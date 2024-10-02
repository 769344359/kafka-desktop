
import DefaultLayout from "@/layouts/default";
import {Button, ButtonGroup} from "@nextui-org/button";
import {Textarea} from "@nextui-org/input";
import  { useState } from 'react';
import {Input} from "@nextui-org/input";
import { invoke } from "@tauri-apps/api/core";
import toast, { Toaster } from 'react-hot-toast';
import {Tabs, Tab} from "@nextui-org/tabs";
import {
  Table,
  TableHeader,
  TableBody,
  TableColumn,
  TableRow,
  TableCell
} from "@nextui-org/table";
export default function IndexPage() {
  const [bootstrap, setBootstrap] = useState('');
  const [topic, setTopic] = useState('');
  const [message,setMessage] = useState('');
  const [isVertical , setIsVertical] = useState(true);
  const [table , setTable] = useState([])
  const [groups,setGroups] = useState([])
  const buttonSubmit = () =>{
    console.log( "topic is" + topic);
    console.log("bootstrap is" + bootstrap);
   let res :Promise<String> =  invoke("send_kafka",{server:bootstrap ,topic:topic,message:message})
   let alltopic  = invoke('get_all_topic_from_server',{server:bootstrap})
   let allgroup = invoke('get_all_group_from_kafka',{server:bootstrap} )
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
  const theStyle={"margin-top":"20px"}
  return (
    
    <DefaultLayout>
       <Tabs aria-label="Options" isVertical={isVertical}>
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
      <TableBody>
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
                <Tab key="groups"  title="Groups" >
                <Table aria-label="Example static collection table">
            <TableHeader>
              <TableColumn>Groups</TableColumn>
              <TableColumn>ROLE</TableColumn>
              <TableColumn>STATUS</TableColumn>
            </TableHeader>
            <TableBody>
            {groups.map(item => (
              <TableRow key={item.name}>
              <TableCell>{item.name}</TableCell>
              <TableCell>{item.members[0].client_id}</TableCell>
              <TableCell>Active</TableCell>
            </TableRow>
              ))}

            </TableBody>
          </Table>
           </Tab>
    </Tabs>
    </DefaultLayout>
  );
}
