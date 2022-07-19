use super::bpm;
use super::sax;
use super::types;

use super::*;

use bimap::BiMap;
use sp_std::vec::Vec;
use sp_std::cell::{Cell, RefCell};
use sp_std::rc::{Rc};

use unborrow::unborrow;

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

    let index = bpmn_str.iter().position(|&c| c == 0x0a).unwrap();
    let bpmn_str = &bpmn_str[index + 1 ..].to_vec();

    let mut process_participants: Rc<RefCell<BiMap<types::Str, types::Actor>>> = Rc::new(RefCell::new(BiMap::new()));
    let mut task_ids_names: Rc<RefCell<BiMap<types::ActionId, types::ActionName>>> = Rc::new(RefCell::new(BiMap::new()));

    {
        let mut process_participants = process_participants.borrow_mut();
        let mut get_process_participants =
        |path: &Vec<&str>, attrs: &Vec<BTreeMap<&str, types::Str>>| {
            match *path.last().unwrap_or(&"") {
                "participant" => {
                    process_participants.insert(
                        attrs.last().unwrap()["processRef"].clone(),
                        attrs.last().unwrap()["name"].clone(),
                    );
                },
                _ => {}
            }
        };
        sax::visit_element_end(&bpmn_str, & mut get_process_participants);
    }

    {
        let mut task_ids_names = task_ids_names.borrow_mut();
        let mut process_participants = process_participants.borrow_mut();
        let mut get_tasks = move |path: &Vec<&str>, attrs: &Vec<BTreeMap<&str, types::Str>>| {
            let name = match *path.last().unwrap_or(&"") {
                "startEvent" | "bpmn:startEvent" => types::from_str("START"),
                "endEvent" | "bpmn:endEvent" => types::from_str("END"),
                "task" | "bpmn:task" => attrs.last().unwrap()["name"].to_vec(),
                _ => types::Str::new()
            };
            let action = types::to_bounded::<T>(name.to_vec());
            if ! action.is_empty() {
                task_ids_names.insert(attrs.last().unwrap()["id"].clone(), name.to_vec());
                let account = if name == types::from_str("START") {
                    <DeProcessOwners<T>>::get(deprocess).unwrap()
                } else {
                    let depth = path.len();
                    let get_name_at = |level: usize| -> types::Str {
                        if depth > level {
                            attrs[level].get("name").unwrap_or(&types::Str::new()).to_vec()
                        } else {
                            types::Str::new()
                        }
                    };
                    access::get_process_action_account::<T>(deprocess, &action, &get_name_at(2), &get_name_at(3), &get_name_at(4))
                };

                <DeProcessActionAccounts<T>>::insert(deprocess, action, account);
            }
        };
        sax::visit_element_end(&bpmn_str, & mut get_tasks);
    }

    {
        let task_ids_names = task_ids_names.borrow();
        let mut get_flows = |path: &Vec<&str>, attrs: &Vec<BTreeMap<&str, types::Str>>|  {
            match *path.last().unwrap_or(&"") {
                "messageFlow" | "bpmn:messageFlow" | "sequenceFlow" | "bpmn:sequenceFlow" => {
                    let src = task_ids_names.get_by_left(&attrs.last().unwrap()["sourceRef"]).unwrap().to_vec();
                    let dst = task_ids_names.get_by_left(&attrs.last().unwrap()["targetRef"]).unwrap().to_vec();
                    <DeModelActionFlows<T>>::insert(deprocess, types::to_bounded::<T>(src), types::to_bounded::<T>(dst));
                },
                _ => {}
            };
        };
        sax::visit_element_end(&bpmn_str, & mut get_flows);
    }

    true
}
