import { Link } from "@nextui-org/link";
import { Snippet } from "@nextui-org/snippet";
import { Code } from "@nextui-org/code";
import { button as buttonStyles } from "@nextui-org/theme";

import { siteConfig } from "@/config/site";
import { title, subtitle } from "@/components/primitives";
import { GithubIcon } from "@/components/icons";
import DefaultLayout from "@/layouts/default";
import {Input} from "@nextui-org/input";
export default function IndexPage() {
  return (
    <DefaultLayout>
    <div className="flex w-full flex-wrap md:flex-nowrap gap-4">
      <Input type="input" label="bootstrap" />
      <Input type="input" label="topic"/>
    </div>
    </DefaultLayout>
  );
}
