var searchIndex = JSON.parse('{\
"fake_socket":{"doc":"","t":[3,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["FakeSocket","ReceiverStream","SenderSink","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","from","from","from","into","into","into","new","new","new","poll_close","poll_close","poll_flush","poll_flush","poll_next","poll_next","poll_ready","poll_ready","start_send","start_send","try_from","try_from","try_from","try_into","try_into","try_into","try_poll_next","try_poll_next","type_id","type_id","type_id"],"q":["fake_socket","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,1,2,3,1,2,3,1,2,3,1,2,3,1,2,3,1,3,1,3,2,3,1,3,1,3,1,2,3,1,2,3,2,3,1,2,3],"f":[null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["unboundedsender",3]]],[[["unboundedreceiver",3]]],[[["unboundedreceiver",3],["unboundedsender",3]]],[[["pin",3],["context",3]],["poll",4,[["result",4]]]],[[["pin",3],["context",3]],["poll",4,[["result",4]]]],[[["pin",3],["context",3]],["poll",4,[["result",4]]]],[[["pin",3],["context",3]],["poll",4,[["result",4]]]],[[["pin",3],["context",3]],["poll",4,[["option",4]]]],[[["pin",3],["context",3]],["poll",4,[["option",4]]]],[[["pin",3],["context",3]],["poll",4,[["result",4]]]],[[["pin",3],["context",3]],["poll",4,[["result",4]]]],[[["pin",3]],["result",4]],[[["pin",3]],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["pin",3],["context",3]],["poll",4,[["option",4,[["result",4]]]]]],[[["pin",3],["context",3]],["poll",4,[["option",4,[["result",4]]]]]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]]],"p":[[3,"SenderSink"],[3,"ReceiverStream"],[3,"FakeSocket"]]},\
"simple_pipeline":{"doc":"","t":[13,13,13,13,13,3,4,3,8,4,13,11,11,11,11,11,11,11,11,10,12,14,14,14,11,12,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,14,11,11,11,11,12,12,12,12,12,12,12,12],"n":["Continue","Err","Internal","InvalidContext","NewPipe","Pipeline","PipelineError","PipelineResponse","Plug","PlugResult","Terminate","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","call","ctx","ctx_mut","ctx_ref","ctx_take","default","err","execute","executed","fmt","fmt","from","from","from","from","into","into","into","into","new","new","new_continue","new_err","new_new_pipe","new_terminate","to_string","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_with","type_id","type_id","type_id","type_id","0","0","0","0","ctx","ctx","err","plugs"],"q":["simple_pipeline","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","simple_pipeline::PipelineError","","simple_pipeline::PlugResult","","","","",""],"d":["","","","","","A sequentially executed pipeline","","Pipeline response","plug trait that the building blocks in a pipeline shall …","execution result for the plug","","","","","","","","","","","","","","","","","execute the entire pipeline sequentially and run to …","","","","","","","","","","","","create a new pipeline","Constructs a new <code>PipelineResponse</code>.","Constructs a new <code>PlugResult::Continue</code>.","Constructs a new <code>PlugResult::Err</code>.","Constructs a new <code>PlugResult::NewPipe</code>.","Constructs a new <code>PlugResult::Terminate</code>.","","","","","","","","","","","","","","","","","","","","","",""],"i":[1,1,2,2,1,0,0,0,0,0,1,2,1,3,4,2,1,3,4,5,4,0,0,0,3,4,3,4,2,2,2,1,3,4,2,1,3,4,3,4,1,1,1,1,2,2,1,3,4,2,1,3,4,0,2,1,3,4,6,7,8,9,10,11,11,10],"f":[null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["pin",3,[["box",3,[["future",8]]]]]],null,null,null,null,[[],["pipeline",3]],null,[[],["pin",3,[["box",3,[["future",8]]]]]],null,[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["vec",3,[["box",3,[["plug",8]]]]],["option",4,[["vec",3,[["string",3]]]]]]],[[["vec",3,[["string",3]]],["option",4,[["pipelineerror",4]]]]],[[]],[[["pipelineerror",4]]],[[["vec",3,[["box",3,[["plug",8]]]]]]],[[]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],null,[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,null,null,null,null,null],"p":[[4,"PlugResult"],[4,"PipelineError"],[3,"Pipeline"],[3,"PipelineResponse"],[8,"Plug"],[13,"InvalidContext"],[13,"Internal"],[13,"Continue"],[13,"Terminate"],[13,"NewPipe"],[13,"Err"]]},\
"simple_pubsub":{"doc":"","t":[3,8,8,11,11,11,11,10,11,10,11,10,11,11,11,11,10],"n":["Broadcaster","Topic","TopicId","borrow","borrow_mut","default","from","get_topic_id","into","publish","remove_subscription","subscribe","topics","try_from","try_into","type_id","unsubscribe"],"q":["simple_pubsub","","","","","","","","","","","","","","","",""],"d":["","","","","","","","","","public a message to a topic","","subscribe to a topic (here name should be session id)","","","","","unsubscribe to a topic"],"i":[0,0,0,1,1,1,1,2,1,3,1,3,1,1,1,1,3],"f":[null,null,null,[[]],[[]],[[]],[[]],[[],["string",3]],[[]],[[["arc",3]]],[[["u32",15]],["option",4,[["u32",15]]]],[[]],[[],["dashmap",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[["u32",15]],["option",4,[["u32",15]]]]],"p":[[3,"Broadcaster"],[8,"TopicId"],[8,"Topic"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};