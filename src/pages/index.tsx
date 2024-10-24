
import DefaultLayout from "@/layouts/default";
import {Button, ButtonGroup} from "@nextui-org/button";
import {Textarea} from "@nextui-org/input";
import  { useEffect, useState } from 'react';
import {Input} from "@nextui-org/input";
// import { invoke } from "@tauri-apps/api/core";
import toast, { Toaster } from 'react-hot-toast';
import {Tabs, Tab} from "@nextui-org/tabs";
import React from "react";
import { invoke, Channel } from '@tauri-apps/api/core';
import { createStore ,Store} from '@tauri-apps/plugin-store';
import store,{setData,SlotResource} from '../store.ts'
import { UseSelector, useSelector } from "react-redux";
import {
  Autocomplete,
  AutocompleteSection,
  AutocompleteItem
} from "@nextui-org/autocomplete";
import {
  Table,
  TableHeader,
  TableBody,
  TableColumn,
  TableRow,
  TableCell
} from "@nextui-org/table";
type KafkaConfig ={
  topic:string,
  server:string,
};

type KafkaMessage ={
  key:string,
  value:string,
  header:string,
  index:number,
  timestamp:number,
  offset:number,
  partition:number,
}

let num = 0;
const onEvent = new Channel<KafkaMessage[]>();
onEvent.onmessage = (messageList:KafkaMessage[]) => {
  console.log("the message is")
  console.log(messageList)
  store.dispatch({type:'counter/setData' ,payload:messageList})
};
function setValue(messageList:KafkaMessage[]){

}







export default function IndexPage() {
  const [consumeTopic,setConsumeTopic] = useState('')
  const [bootstrap, setBootstrap] = useState('');
  const [topic, setTopic] = useState('');
  const [message,setMessage] = useState('');
  const [isVertical , setIsVertical] = useState(true);
  const [table , setTable] = useState([])
  const [groups,setGroups] = useState([])
  const [kafkaMessage,setKafkaMessage] = useState<KafkaMessage>([])
  const [topicList,setTopicList] = useState([]);
  const [selectedTopic,setSelectedTopic] = useState('')
  // const vv = useSelector( (one) => one.value)
  const storeAware = createStore('store1.bin', {
    // we can save automatically after each store modification
  });

  console.log("aba")
  storeAware.then((store) =>{
    store.get<KafkaConfig>("1").then((value) =>{
      num++;
      if(value != null && num == 2){

        console.log("modify---"+num)
        console.log(value)
        setTopic(value.topic)
        setBootstrap(value.server)
      }
    })
  })
  const startConsumer = () =>{
    let resource :SlotResource={
      slotNum:1,
      server:bootstrap,
    } 
    let  consumeRes =  invoke("consume_kafka",{resource:resource,  config:{ server:bootstrap ,topic:consumeTopic}, onEvent})
    consumeRes.then((list) =>{
      setKafkaMessage(list)
    })

  }
  store.subscribe(() => {
    console.log("fffab")
    let tem = JSON.parse(JSON.stringify(store.getState().value))
    console.log(tem)
    setKafkaMessage(tem)
  }
)
  const tryGetAllTopic = ()=>{
    let topicPromise  = invoke('get_all_topic_from_server',{server:bootstrap})
    topicPromise.then((re) =>{
       let tem =  re.topics;
       console.log("result")
       console.log(re)
       setTopicList(tem)
    });
  }
  const buttonSubmit = () =>{
    console.log( "topic is" + topic);
    console.log("bootstrap is" + bootstrap);
  
    storeAware.then( (store)=>{
      let config :KafkaConfig ={
        server:bootstrap,
        topic:topic
      };
      store.set("1",config)
      store.save()
    })

   let res :Promise<String> =  invoke("send_kafka",{server:bootstrap ,topic:topic,message:message})
   let resource :SlotResource={
     slotNum:1,
     server:bootstrap,
   } 
   let alltopic  = invoke('get_all_topic_from_server',resource)
   let allgroup = invoke('get_all_group_from_kafka',{server:bootstrap} )
   tryGetAllTopic()
   alltopic.then(re=>{
    setTable(re.topics)
     console.log("aaaaa" + JSON.stringify(re.topics))
   })
   res.then((m) =>{ 
    toast('Here is your toast. ' + m )
   })
   allgroup.then(re =>{
    setGroups(re.groups)
    console.log("aba" + JSON.stringify(re))
   })
}
  const theStyle={"marginTop":"20px"}
  const widthFull =  {"width":"100%"}
  // const  tabClassNames = {panel:{"width":"100%"}};
  const classNames = React.useMemo(
    () => ({
      panel: ["w-full"],

    }),
    [],
  );
  return (
    
    <DefaultLayout>
       <Tabs aria-label="Options" isVertical={isVertical} classNames={classNames}>
          <Tab key="send_message" title="Send">
              <div className="flex w-full flex-wrap md:flex-nowrap gap-4">
                <Input type="input" label="bootstrap server" value={bootstrap} onValueChange={setBootstrap} />
                <Input type="input" label="topic" value={topic} onValueChange={setTopic} />
              </div>
              <div style={theStyle}>
              <Textarea
                label="Description"
                placeholder="Enter your description"
                // className="max-w-xs"
              />
              </div>
              <Toaster />
              <div style={theStyle}>
              <Button color="primary" onClick={buttonSubmit}>
                Send
              </Button>
              </div>
           </Tab>
           <Tab key="topic" title="Topic">
           <Table aria-label="Example static collection table">
      <TableHeader>
        <TableColumn>Topic</TableColumn>
        <TableColumn>ROLE</TableColumn>
        <TableColumn>STATUS</TableColumn>
      </TableHeader>
      <TableBody >
      {table.map(item => (
        <TableRow key={item}>
        <TableCell>{item}</TableCell>
        <TableCell>CEO</TableCell>
        <TableCell>Active</TableCell>
      </TableRow>
        ))}

      </TableBody>
          </Table>
          </Tab>
          <Tab key="groups"  title="Groups"    >
 
                <Table   aria-label="Example static collection table">
            <TableHeader>
              <TableColumn>Groups</TableColumn>
              <TableColumn>ClientId</TableColumn>
              <TableColumn>STATUS</TableColumn>
            </TableHeader>
            <TableBody>
            {groups.map(item => (
              <TableRow key={item?.name}>
              <TableCell>{item?.name}</TableCell>
              <TableCell>{item?.name}</TableCell>
              {/* <TableCell>{item?.members[0]?.client_id}</TableCell> */}
              <TableCell>Active</TableCell>
            </TableRow>
              ))}

            </TableBody>
          </Table>
           </Tab>
           <Tab key="consumer"  title="Consumer"    >
           <Autocomplete 
        label="Select an animal" 
        className="max-w-xs" 
        selectedKey={consumeTopic}
        onSelectionChange={setConsumeTopic}
      >
        {topicList.map((animal) => (
          <AutocompleteItem key={animal} value={animal}>
            {animal}
          </AutocompleteItem>
        ))}
      </Autocomplete>
           <Button color="primary" onClick={startConsumer}>
            Start
           </Button>
           
                <Table   aria-label="Example static collection table">
            <TableHeader>
              <TableColumn>Key</TableColumn>
              <TableColumn>Value</TableColumn>
              <TableColumn>Header</TableColumn>
              <TableColumn>Timestamp</TableColumn>
              <TableColumn>Offset</TableColumn>
              <TableColumn>Partition</TableColumn>
            </TableHeader>
            <TableBody>
            {kafkaMessage.map(item => (
              <TableRow key={item.index}>
              <TableCell>{item.key}</TableCell>
              <TableCell>{item.value}</TableCell>
              {/* <TableCell>{item.members[0].client_id}</TableCell> */}
              <TableCell>{item.header}</TableCell>
              <TableCell>{item.timestamp}</TableCell>
              <TableCell>{item.offset}</TableCell>
              <TableCell>{item.partition}</TableCell>
            </TableRow>
              ))}

            </TableBody>
          </Table>
           </Tab>
    </Tabs>
    </DefaultLayout>
  );
}
