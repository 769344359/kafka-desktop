
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
  const buttonSubmit = () =>{
    console.log( "topic is" + topic);
    console.log("bootstrap is" + bootstrap);
   let res :Promise<String> =  invoke("send_kafka",{server:bootstrap ,topic:topic,message:message})
   res.then((m) =>{ 
    toast('Here is your toast. ' + m )
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
        <TableColumn>NAME</TableColumn>
        <TableColumn>ROLE</TableColumn>
        <TableColumn>STATUS</TableColumn>
      </TableHeader>
      <TableBody>
        <TableRow key="1">
          <TableCell>Tony Reichert</TableCell>
          <TableCell>CEO</TableCell>
          <TableCell>Active</TableCell>
        </TableRow>
        <TableRow key="2">
          <TableCell>Zoey Lang</TableCell>
          <TableCell>Technical Lead</TableCell>
          <TableCell>Paused</TableCell>
        </TableRow>
        <TableRow key="3">
          <TableCell>Jane Fisher</TableCell>
          <TableCell>Senior Developer</TableCell>
          <TableCell>Active</TableCell>
        </TableRow>
        <TableRow key="4">
          <TableCell>William Howard</TableCell>
          <TableCell>Community Manager</TableCell>
          <TableCell>Vacation</TableCell>
        </TableRow>
      </TableBody>
    </Table>
           </Tab>
    </Tabs>
    </DefaultLayout>
  );
}
