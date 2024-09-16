import { Link } from "@nextui-org/link";
import { Snippet } from "@nextui-org/snippet";
import { Code } from "@nextui-org/code";
import { button as buttonStyles } from "@nextui-org/theme";

import { siteConfig } from "@/config/site";
import { title, subtitle } from "@/components/primitives";
import { GithubIcon } from "@/components/icons";
import DefaultLayout from "@/layouts/default";
import {Button, ButtonGroup} from "@nextui-org/button";
import {Textarea} from "@nextui-org/input";
import  { useState } from 'react';
import {Input} from "@nextui-org/input";
import { invoke } from "@tauri-apps/api/core";

export default function IndexPage() {
  const [bootstrap, setBootstrap] = useState('');
  const [topic, setTopic] = useState('');
  const buttonSubmit = () =>{
    console.log( "topic is" + topic);
    console.log("bootstrap is" + bootstrap);
    invoke("send_kafka",{server:bootstrap ,topic:topic,message:"hihi"})

  }
  return (
    <DefaultLayout>
    <div className="flex w-full flex-wrap md:flex-nowrap gap-4">
      <Input type="input" label="bootstrap server" value={bootstrap} onValueChange={setBootstrap} />
      <Input type="input" label="topic" value={topic} onValueChange={setTopic} />
    </div>
    <div>
    <Textarea
      label="Description"
      placeholder="Enter your description"
      className="max-w-xs"
    />
    </div>
    <div>
    <Button color="primary" onClick={buttonSubmit}>
      Send
    </Button>
    </div>
    </DefaultLayout>
  );
}
