use super::bpm;
use super::sax;
use super::types;

use super::*;

use bimap::BiMap;
use sp_std::vec::Vec;
use sp_std::cell::{Cell, RefCell};
use sp_std::rc::{Rc};

use unborrow::unborrow;

use sp_std::str::FromStr;

pub type BpmnStr = types::Str;

pub enum ActorType {
    lane = -2,
    laneSet =  -3,
    process = -4,
}

impl ActorType {
    fn as_str(&self) -> types::Str {
        types::from_str(match self {
            ActorType::lane => "lane",
            ActorType::laneSet => "laneSet",
            ActorType::process => "process",
        })
    }

}

pub fn store_model_spec_bpmn<T: Config>(deprocess: &types::DeProcessId, bpmn_str: &BpmnStr) -> bool {

    fn last<'a>(path: &'a Vec<&str>) -> &'a str {
        let last: &str = *path.last().unwrap();
        let start = if last.starts_with("bpmn:") {
            5
        } else {
			0
		};
		let last = &last[start ..];
        last
    }

    {
        let mut get_process_participants =
        |path: &Vec<&str>, attrs: &Vec<BTreeMap<&str, types::Str>>| {
            let el_name = last(path);
            let el_attrs = attrs.last().unwrap();
            let typ = bpm::NodeType::from_str(el_name).unwrap();
            let depth = path.len();

            let id = match typ {
                bpm::NodeType::Participant => el_attrs["processRef"].clone(),
                _ => types::str_unwrap_default(el_attrs.get("id")).clone(),
            };
            let name = match typ {
                bpm::NodeType::Start => types::from_str("START"),
                bpm::NodeType::End => types::from_str("END"),
                _ => types::str_unwrap_default(el_attrs.get("name")).to_vec(),
            };
            
            {
                let id = types::to_bounded::<T>(id.to_vec());
                let name_s = types::to_bounded::<T>(name.to_vec());

                if !id.is_empty() {
                    <DeModelNodes<T>>::insert(deprocess, id.clone(), (typ.clone(), name_s.clone()));
                    if bpm::NodeType::Task == typ || bpm::NodeType::Start == typ || bpm::NodeType::End == typ {
                        <DeModelNameId<T>>::insert(deprocess, name_s.clone(), id.clone()); //reverse lookup;
                    }
                };

                if bpm::NodeType::SequenceFlow == typ || bpm::NodeType::MessageFlow == typ {
                    let attrs = attrs.last().unwrap();
                    let branch = U256::from_dec_str(types::to_str(&name)).unwrap_or_default();

                    let src_id = types::to_bounded::<T>(el_attrs["sourceRef"].to_vec());
                    let dst_id = types::to_bounded::<T>(attrs["targetRef"].to_vec());

                    <DeModelEdges<T>>::insert((deprocess, src_id, branch), (id, dst_id,));
                }
            }

            if bpm::NodeType::Task == typ {

                let participant = types::str_unwrap_default(attrs[1].get("name")).to_vec();
                let lane = if depth > 3 {
                    types::str_unwrap_default(attrs[2].get("name")).to_vec()
                } else {
                    types::Str::default()
                };
                let lane_set = if depth > 4 {
                    types::str_unwrap_default(attrs[3].get("name"))
                } else {
                    types::Str::default()
                };

                let account = access::get_process_action_account::<T>(deprocess, &name, &participant, &lane, &lane_set);
                let id = types::to_bounded::<T>(id);
                <DeProcessActionAccounts<T>>::insert(deprocess, id, account);
            }
        };
        sax::visit_element_end(&bpmn_str, & mut get_process_participants);
    }

    true
}
