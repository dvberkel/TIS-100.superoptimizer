var searchIndex = {};
searchIndex["tis_100_superoptimizer"] = {"doc":"[TIS-100.superoptimizer](https://github.com/dvberkel/TIS-100.superoptimizer) is a","items":[[0,"TIS_100","tis_100_superoptimizer","The Tessellated Intelligence System is a",null,null],[3,"Node","tis_100_superoptimizer::TIS_100","A `Node` models the basic execution node in TIS-100. You change a node state\nby running `Program`s on it or executing an `Instruction` on it.",null,null],[12,"acc","","The accumulator for the basic execution node.",0,null],[12,"up","","The up `Port` used for reading",0,null],[12,"down","","The down `Port` used for writing",0,null],[3,"Program","","A `Program` is a sequence of `Instruction`s",null,null],[12,"0","","",1,null],[4,"Instruction","","`Instruction`s are executed by a `Node`",null,null],[13,"NOP","","Does nothing",2,null],[13,"MOV","","Moves value from `Source` to `Destination`",2,null],[13,"SWP","","Swaps the value of the accumulator (acc) and the backup (bac) register",2,null],[13,"SAV","","Saves the value of the accumulator (acc) to the backup register",2,null],[13,"ADD","","Add value from `Source` to accumulator (acc), storing result in acc",2,null],[13,"SUB","","Subtracts value from `Source` from accumulator (acc), storing result in acc",2,null],[4,"Source","","`Source` are either ports, registers or literals",null,null],[13,"Port","","A port, will always be the up port",3,null],[13,"Register","","A register",3,null],[13,"Literal","","A literal value",3,null],[4,"Register","","Different types of Registers known in TIS-100",null,null],[13,"NIL","","the NIL register, reading from it provides with zero",4,null],[13,"ACC","","The accumulator",4,null],[4,"Destination","","`Destination` are either ports or registers",null,null],[13,"Port","","A Port, will always be the DOWN port",5,null],[13,"Register","","A register",5,null],[4,"Status","","The `Status` a `Program` run on a certain `Node`.",null,null],[13,"Successful","","a successful run of the program",6,null],[13,"Deadlock","","an unsuccessful run of the program, because of a deadlock.",6,null],[0,"Ports","","`Ports` are the IO channels for a `Node`",null,null],[3,"Port","tis_100_superoptimizer::TIS_100::Ports","A `Port` can be read from and write to",null,null],[4,"PortReadResult","","The result of reading from a `Port`",null,null],[13,"Success","","Success",7,null],[13,"Failure","","Failure",7,null],[11,"clone","","",8,null],[11,"eq","","",8,null],[11,"ne","","",8,null],[11,"fmt","","",8,null],[11,"fmt","","",7,null],[11,"new","","Create a port with a number of readable values",8,{"inputs":[{"name":"vec"}],"output":{"name":"port"}}],[11,"with","","Create a port with prescribed input and output",8,{"inputs":[{"name":"vec"},{"name":"vec"}],"output":{"name":"port"}}],[11,"read","","Read from this `Port`. Will return a `PortReadResult::Success` when a\nvalue is available, otherwise a `PortReadResult::Failure`",8,null],[11,"write","","Write to this `Port`. Will always succeed and return the Port as it is\nchanged by the write",8,null],[11,"eq","tis_100_superoptimizer::TIS_100","",0,null],[11,"ne","","",0,null],[11,"fmt","","",0,null],[11,"eq","","",1,null],[11,"fmt","","",1,null],[11,"clone","","",1,null],[11,"clone","","",2,null],[11,"eq","","",2,null],[11,"ne","","",2,null],[11,"fmt","","",2,null],[11,"clone","","",3,null],[11,"eq","","",3,null],[11,"ne","","",3,null],[11,"fmt","","",3,null],[11,"clone","","",4,null],[11,"eq","","",4,null],[11,"fmt","","",4,null],[11,"clone","","",5,null],[11,"eq","","",5,null],[11,"ne","","",5,null],[11,"fmt","","",5,null],[11,"new","","Create a `Node` with defaults for accumulator, backup registers, program counter and program",0,{"inputs":[],"output":{"name":"node"}}],[11,"load","","Loads a program in this `Node`",0,null],[11,"run","","Run the loaded program, returning an calculation state",0,null],[11,"execute","","Execute the `instruction` on this `Node`. Returns a `Node` that reflects\nthe changes the `instruction` would have on this `Node`.",0,null]],"paths":[[3,"Node"],[3,"Program"],[4,"Instruction"],[4,"Source"],[4,"Register"],[4,"Destination"],[4,"Status"],[4,"PortReadResult"],[3,"Port"]]};
initSearch(searchIndex);
