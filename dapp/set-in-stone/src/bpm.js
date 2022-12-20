// path = "/bookstore/book/price[text()]";

function visit(bpmn, tag, visitor) {
    let els = bpmn.getElementsByTagName(tag);
    for(var el of els) {
        visitor(el, tag);
    }
}

// import idl from './next_step_demo.json';

export const NodeType = {
	Undefined: 0,
	None: 1,
	Start: 2,
	End: 3,
	Task: 4,
	Script: 5,
	InclusiveGateway: 6,
	ExclusiveGateway: 7,
	ParallelGateway: 8,
	EventBasedGateway: 9,
	ComplexGateway: 10,
	Process: 11,
	Participant: 12,
	LaneSet: 13,
	Lane: 14,
	SequenceFlow: 15,
	MessageFlow: 16,
};

export function parse_bpmn(bpmn) {

    //task names, ids

    var participants_names_ids = {};
    var actions_names_ids = {};
    var nodes = {};
    var flows = {};

    function store_participants_names_ids(el, _) {
        participants_names_ids[el.getAttribute("name")] = hash(el.getAttribute("id"));
    }
    visit(bpmn, "participant", store_participants_names_ids);

    function store_actions_names_ids(el, _) {
        actions_names_ids[el.getAttribute("name")] = hash(el.getAttribute("id"));
    }
    visit(bpmn, "task", store_actions_names_ids);

    function store_nodes(el, tag) {
        actions_names_ids[el.getAttribute("name")] = hash(el.getAttribute("id"));

        let hasLevel = false;
        var p = el.parentElement;
        
        hasLevel = "lane" == p.tagName;
        let lane = hasLevel ? p.getAttribute("name") : null;
        if(hasLevel) {
            p = el.parentElement;
        }
        
        hasLevel = "laneSet" == p.tagName;
        let laneSet = hasLevel ? p.getAttribute("name") : null;
        if(hasLevel) {
            p = el.parentElement;
        }
        
        let process = p.getAttribute("name");

        var typ;
        var name;

        var ref_id = null;

        switch(tag) {
            case "task":
                typ = NodeType.Task;
                name = el.getAttribute("name");
                break;

            case "startEvent":
                typ = NodeType.Start;
                name = "START";
                break;

            case "endEvent":
                typ = NodeType.End;
                name = "END";
                break;

            case "InclusiveGateway":
                typ = NodeType.InclusiveGateway;
                name = el.getAttribute("name");
                break;

            case "ExclusiveGateway":
                typ = NodeType.ExclusiveGateway;
                name = el.getAttribute("name");
                ref_id = actions_names_ids[name];
                break;
        };

        let id = hash(el.getAttribute("id"));

        let node = {
            id: id,
            name: name,
            typ: typ,
            ref_id: ref_id,
            heir: [process, laneSet, lane]
        }

        nodes[id] = node;
    } 
    visit(bpmn, "task", store_nodes);
    visit(bpmn, "startEvent", store_nodes);
    visit(bpmn, "endEvent", store_nodes);
    visit(bpmn, "inclusiveGateway", store_nodes);
    visit(bpmn, "exclusiveGateway", store_nodes);

    function store_flows(el, tag) {
        let id = hash(el.getAttribute("id"));
        let branch = el.hasAttribute("name") ? parseInt(el.getAttribute("name")) : 0;
        let flow = {
            id: id,
            src: hash(el.getAttribute("sourceRef")),
            dst: hash(el.getAttribute("targetRef")),
            branch: branch
        }

        flows[id] = flow;
    } 
    visit(bpmn, "sequenceFlow", store_flows);
    visit(bpmn, "messageFlow", store_flows);
    
    return [nodes, flows];
}

