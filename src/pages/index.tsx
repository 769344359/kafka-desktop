
import DefaultLayout from "@/layouts/default";
import {Button, ButtonGroup} from "@nextui-org/button";
import {Textarea} from "@nextui-org/input";
import  { useState } from 'react';
import {Input} from "@nextui-org/input";
import { invoke } from "@tauri-apps/api/core";
import toast, { Toaster } from 'react-hot-toast';
import {Tabs, Tab} from "@nextui-org/tabs";
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

           </Tab>
    </Tabs>
    </DefaultLayout>
  );
}
