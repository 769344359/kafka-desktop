import { Route, Routes } from "react-router-dom";

import IndexPage from "@/pages/index";
import DocsPage from "@/pages/docs";
import PricingPage from "@/pages/pricing";
import BlogPage from "@/pages/blog";
import AboutPage from "@/pages/about";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Provider } from 'react-redux';
import store from './store';
function App() {
  // const [greetMsg, setGreetMsg] = useState("");
  // const [name, setName] = useState("");

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   // setGreetMsg(await invoke("greet", { name }));
  //   setGreetMsg(await invoke("my_custom_command"));
    
  // }

  // return (
  //   <div className="container">
  //     <h1>Welcome to Tauri!</h1>

  //     <div className="row">
  //       <a href="https://vitejs.dev" target="_blank">
  //         <img src="/vite.svg" className="logo vite" alt="Vite logo" />
  //       </a>
  //       <a href="https://tauri.app" target="_blank">
  //         <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
  //       </a>
  //     </div>

  //     <p>Click on the Tauri, Vite, and React logos to learn more.</p>

  //     <form
  //       className="row"
  //       onSubmit={(e) => {
  //         e.preventDefault();
  //         greet();
  //       }}
  //     >
  //       <input
  //         id="greet-input"
  //         onChange={(e) => setName(e.currentTarget.value)}
  //         placeholder="Enter a name..."
  //       />
  //       <button type="submit">Greet</button>
  //     </form>

  //     <p>{greetMsg}</p>
  //   </div>
  // );
  return (
    <Routes>

      <Route element={<IndexPage />} path="/" />

      <Route element={<DocsPage />} path="/docs" />
      <Route element={<PricingPage />} path="/pricing" />
      <Route element={<BlogPage />} path="/blog" />
      <Route element={<AboutPage />} path="/about" />
    </Routes>
  );
}

export default App;
