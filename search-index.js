var searchIndex = {};
searchIndex["tis_100_superoptimizer"] = {"doc":"[TIS-100.superoptimizer](https://github.com/dvberkel/TIS-100.superoptimizer) is a","items":[[0,"TIS_100","tis_100_superoptimizer","The Tessellated Intelligence System is a",null,null],[3,"Node","tis_100_superoptimizer::TIS_100","A `Node` models the basic execution node in TIS-100. You change a node state\nby running `Program`s on it or executing an `Instruction` on it.",null,null],[12,"acc","","The accumulator for the basic execution node.",0,null],[12,"up","","The up `Port` used for reading",0,null],[12,"down","","The down `Port` used for writing",0,null],[3,"Program","","A `Program` is a sequence of `Instruction`s",null,null],[12,"0","","",1,null],[4,"Instruction","","`Instruction`s are executed by a `Node`",null,null],[13,"NOP","","Does nothing",2,null],[13,"MOV","","Moves value from `Source` to `Destination`",2,null],[13,"SWP","","Swaps the value of the accumulator (acc) and the backup (bac) register",2,null],[13,"SAV","","Saves the value of the accumulator (acc) to the backup register",2,null],[13,"ADD","","Add value from `Source` to accumulator (acc), storing result in acc",2,null],[13,"SUB","","Subtracts value from `Source` from accumulator (acc), storing result in acc",2,null],[4,"Source","","`Source` are either ports, registers or literals",null,null],[13,"Port","","A port, will always be the up port",3,null],[13,"Register","","A register",3,null],[13,"Literal","","A literal value",3,null],[4,"Register","","Different types of Registers known in TIS-100",null,null],[13,"NIL","","the NIL register, reading from it provides with zero",4,null],[13,"ACC","","The accumulator",4,null],[4,"Destination","","`Destination` are either ports or registers",null,null],[13,"Port","","A Port, will always be the DOWN port",5,null],[13,"Register","","A register",5,null],[4,"ErrorStatus","","The `ErrorStatus` a `Program` of an unsuccessful run on a certain `Node`",null,null],[13,"Deadlock","","a deadlock occurred",6,null],[13,"Timeout","","a timeout occurred",6,null],[4,"Cycle","","Determines how many cycles to run a program",null,null],[13,"Indefinetly","","Run until the input arguments run out",7,null],[13,"Maximum","","Run for a maximum of cycles",7,null],[0,"Ports","","`Ports` are the IO channels for a `Node`",null,null],[3,"Port","tis_100_superoptimizer::TIS_100::Ports","A `Port` can be read from and write to",null,null],[12,"input","","The input port",8,null],[12,"output","","the output port",8,null],[11,"clone","","",8,null],[11,"eq","","",8,null],[11,"ne","","",8,null],[11,"fmt","","",8,null],[11,"new","","Create a port with a number of readable values",8,{"inputs":[{"name":"vec"}],"output":{"name":"port"}}],[11,"with","","Create a port with prescribed input and output",8,{"inputs":[{"name":"vec"},{"name":"vec"}],"output":{"name":"port"}}],[11,"read","","Read from this `Port`. Will return a `PortReadResult::Success` when a\nvalue is available, otherwise a `PortReadResult::Failure`",8,null],[11,"available","","Determine if self is available for reading",8,null],[11,"write","","Write to this `Port`. Will always succeed and return the Port as it is\nchanged by the write",8,null],[11,"clone","tis_100_superoptimizer::TIS_100","",0,null],[11,"eq","","",0,null],[11,"ne","","",0,null],[11,"fmt","","",0,null],[11,"eq","","",1,null],[11,"fmt","","",1,null],[11,"clone","","",1,null],[11,"clone","","",2,null],[11,"eq","","",2,null],[11,"ne","","",2,null],[11,"fmt","","",2,null],[11,"clone","","",3,null],[11,"eq","","",3,null],[11,"ne","","",3,null],[11,"fmt","","",3,null],[11,"clone","","",4,null],[11,"eq","","",4,null],[11,"fmt","","",4,null],[11,"clone","","",5,null],[11,"eq","","",5,null],[11,"ne","","",5,null],[11,"fmt","","",5,null],[11,"new","","Create a `Node` with defaults for accumulator, backup registers, program counter and program",0,{"inputs":[],"output":{"name":"node"}}],[11,"load","","Loads a program in this `Node`",0,null],[11,"run","","Run the loaded program, returning an calculation state",0,null],[11,"set_up","","Create a `Node` from self with a prescribed down port",0,null],[11,"execute","","Execute the `instruction` on this `Node`. Returns a `Node` that reflects\nthe changes the `instruction` would have on this `Node`.",0,null],[0,"check","tis_100_superoptimizer","The check module verifies if a certain `Program` when run on a specific\n`Node` produces the expected result. I.e. Reads the input on `Source::Port`\nand writes the correct sequence the `Destination::Port`.",null,null],[5,"check","tis_100_superoptimizer::check","Checks if `Node` when run with `Program` writes `expected_result` to `Destination::Port`",null,{"inputs":[{"name":"node"},{"name":"program"},{"name":"vec"},{"name":"u32"}],"output":{"name":"bool"}}],[0,"iterator","tis_100_superoptimizer","The `optimizer` module contains iterators that allow a user to iterate over\n`Program`s in order of increasing content",null,null],[3,"ProgramIterator","tis_100_superoptimizer::iterator","Iterator over Programs",null,null],[8,"Content","","Allows one to determine the content of certain constructs, e.g. `Program`s\nor `Instruction`s",null,null],[10,"content","","the content of the construct",9,null],[11,"content","tis_100_superoptimizer::TIS_100","",1,null],[11,"content","","",2,null],[11,"content","","",3,null],[11,"content","","",5,null],[11,"new","tis_100_superoptimizer::iterator","Create a `ProgramIterator`",10,{"inputs":[],"output":{"name":"programiterator"}}],[11,"next","","",10,null],[0,"optimizer","tis_100_superoptimizer","Will find the shortest program to problem",null,null],[3,"Config","tis_100_superoptimizer::optimizer","Configuration for the optimize function",null,null],[12,"maximum_cycle","","The maximum allowed number of cycles per program",11,null],[12,"maximum_program_length","","The maximum allowed program length",11,null],[5,"optimize","","Tries to find a `Program` that satisfies the context",null,{"inputs":[{"name":"node"},{"name":"vec"},{"name":"config"}],"output":{"name":"option"}}],[11,"new","","create a `Config` with prescribed maximum_cycle and maximum_program_length",11,{"inputs":[{"name":"u32"},{"name":"usize"}],"output":{"name":"config"}}]],"paths":[[3,"Node"],[3,"Program"],[4,"Instruction"],[4,"Source"],[4,"Register"],[4,"Destination"],[4,"ErrorStatus"],[4,"Cycle"],[3,"Port"],[8,"Content"],[3,"ProgramIterator"],[3,"Config"]]};
searchIndex["yaml_rust"] = {"doc":"YAML 1.2 implementation in pure Rust.","items":[[0,"yaml","yaml_rust","",null,null],[3,"YamlLoader","yaml_rust::yaml","",null,null],[4,"Yaml","","A YAML node is stored as this `Yaml` enumeration, which provides an easy way to\naccess your YAML document.",null,null],[13,"Real","","Float types are stored as String and parsed on demand.\nNote that f64 does NOT implement Eq trait and can NOT be stored in BTreeMap.",0,null],[13,"Integer","","YAML int is stored as i64.",0,null],[13,"String","","YAML scalar.",0,null],[13,"Boolean","","YAML bool, e.g. `true` or `false`.",0,null],[13,"Array","","YAML array, can be accessed as a `Vec`.",0,null],[13,"Hash","","YAML hash, can be accessed as a `BTreeMap`.",0,null],[13,"Alias","","Alias, not fully supported yet.",0,null],[13,"Null","","YAML null, e.g. `null` or `~`.",0,null],[13,"BadValue","","Accessing a nonexistent node via the Index trait returns `BadValue`. This\nsimplifies error handling in the calling code. Invalid type conversion also\nreturns `BadValue`.",0,null],[6,"Array","","",null,null],[6,"Hash","","",null,null],[11,"hash","","",0,null],[11,"cmp","","",0,null],[11,"fmt","","",0,null],[11,"partial_cmp","","",0,null],[11,"lt","","",0,null],[11,"le","","",0,null],[11,"gt","","",0,null],[11,"ge","","",0,null],[11,"eq","","",0,null],[11,"ne","","",0,null],[11,"clone","","",0,null],[11,"on_event","","",1,null],[11,"load_from_str","","",1,{"inputs":[{"name":"str"}],"output":{"name":"result"}}],[11,"as_bool","","",0,null],[11,"as_i64","","",0,null],[11,"as_str","","",0,null],[11,"as_hash","","",0,null],[11,"as_vec","","",0,null],[11,"is_null","","",0,null],[11,"is_badvalue","","",0,null],[11,"as_f64","","",0,null],[11,"from_str","","",0,{"inputs":[{"name":"str"}],"output":{"name":"yaml"}}],[11,"index","","",0,null],[11,"index","","",0,null],[0,"scanner","yaml_rust","",null,null],[3,"Marker","yaml_rust::scanner","",null,null],[3,"ScanError","","",null,null],[3,"Token","","",null,null],[12,"0","","",2,null],[12,"1","","",2,null],[3,"Scanner","","",null,null],[4,"TEncoding","","",null,null],[13,"Utf8","","",3,null],[4,"TScalarStyle","","",null,null],[13,"Any","","",4,null],[13,"Plain","","",4,null],[13,"SingleQuoted","","",4,null],[13,"DoubleQuoted","","",4,null],[13,"Literal","","",4,null],[13,"Foled","","",4,null],[4,"TokenType","","",null,null],[13,"NoToken","","",5,null],[13,"StreamStart","","",5,null],[13,"StreamEnd","","",5,null],[13,"VersionDirective","","major, minor",5,null],[13,"TagDirective","","handle, prefix",5,null],[13,"DocumentStart","","",5,null],[13,"DocumentEnd","","",5,null],[13,"BlockSequenceStart","","",5,null],[13,"BlockMappingStart","","",5,null],[13,"BlockEnd","","",5,null],[13,"FlowSequenceStart","","",5,null],[13,"FlowSequenceEnd","","",5,null],[13,"FlowMappingStart","","",5,null],[13,"FlowMappingEnd","","",5,null],[13,"BlockEntry","","",5,null],[13,"FlowEntry","","",5,null],[13,"Key","","",5,null],[13,"Value","","",5,null],[13,"Alias","","",5,null],[13,"Anchor","","",5,null],[13,"Tag","","handle, suffix",5,null],[13,"Scalar","","",5,null],[6,"ScanResult","","",null,null],[11,"fmt","","",3,null],[11,"eq","","",3,null],[11,"clone","","",3,null],[11,"fmt","","",4,null],[11,"eq","","",4,null],[11,"clone","","",4,null],[11,"fmt","","",6,null],[11,"eq","","",6,null],[11,"ne","","",6,null],[11,"clone","","",6,null],[11,"fmt","","",7,null],[11,"eq","","",7,null],[11,"ne","","",7,null],[11,"clone","","",7,null],[11,"new","","",7,{"inputs":[{"name":"marker"},{"name":"str"}],"output":{"name":"scanerror"}}],[11,"description","","",7,null],[11,"cause","","",7,null],[11,"fmt","","",7,null],[11,"fmt","","",5,null],[11,"eq","","",5,null],[11,"ne","","",5,null],[11,"clone","","",5,null],[11,"fmt","","",2,null],[11,"eq","","",2,null],[11,"ne","","",2,null],[11,"clone","","",2,null],[11,"fmt","","",8,null],[11,"next","","",8,null],[11,"new","","Creates the YAML tokenizer.",8,{"inputs":[{"name":"t"}],"output":{"name":"scanner"}}],[11,"get_error","","",8,null],[11,"stream_started","","",8,null],[11,"stream_ended","","",8,null],[11,"mark","","",8,null],[11,"fetch_next_token","","",8,null],[11,"next_token","","",8,null],[11,"fetch_more_tokens","","",8,null],[0,"parser","yaml_rust","",null,null],[3,"Parser","yaml_rust::parser","",null,null],[4,"Event","","`Event` is used with the low-level event base parsing API,\nsee `EventReceiver` trait.",null,null],[13,"NoEvent","","Reserved for internal use",9,null],[13,"StreamStart","","",9,null],[13,"StreamEnd","","",9,null],[13,"DocumentStart","","",9,null],[13,"DocumentEnd","","",9,null],[13,"Alias","","Refer to an anchor ID",9,null],[13,"Scalar","","Value, style, anchor_id, tag",9,null],[13,"SequenceStart","","Anchor ID",9,null],[13,"SequenceEnd","","",9,null],[13,"MappingStart","","Anchor ID",9,null],[13,"MappingEnd","","",9,null],[6,"ParseResult","","",null,null],[8,"EventReceiver","","",null,null],[10,"on_event","","",10,null],[11,"fmt","","",9,null],[11,"eq","","",9,null],[11,"ne","","",9,null],[11,"clone","","",9,null],[11,"fmt","","",11,null],[11,"new","","",11,{"inputs":[{"name":"t"}],"output":{"name":"parser"}}],[11,"load","","",11,null],[0,"emitter","yaml_rust","",null,null],[3,"YamlEmitter","yaml_rust::emitter","",null,null],[4,"EmitError","","",null,null],[13,"FmtError","","",12,null],[13,"BadHashmapKey","","",12,null],[6,"EmitResult","","",null,null],[11,"fmt","","",12,null],[11,"clone","","",12,null],[11,"from","","",12,{"inputs":[{"name":"error"}],"output":{"name":"self"}}],[11,"new","","",13,{"inputs":[{"name":"write"}],"output":{"name":"yamlemitter"}}],[11,"dump","","",13,null]],"paths":[[4,"Yaml"],[3,"YamlLoader"],[3,"Token"],[4,"TEncoding"],[4,"TScalarStyle"],[4,"TokenType"],[3,"Marker"],[3,"ScanError"],[3,"Scanner"],[4,"Event"],[8,"EventReceiver"],[3,"Parser"],[4,"EmitError"],[3,"YamlEmitter"]]};
initSearch(searchIndex);
