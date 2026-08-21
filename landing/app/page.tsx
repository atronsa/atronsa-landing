import Hero from "@/components/Hero";
import Logo from "@/components/Logo";
import WhatWeDo from "@/components/WhatWeDo";
import WebResult from "@/components/WebResult";
import Pricing from "@/components/Pricing";
import WhatWeBuild from "@/components/WhatWeBuild";
import FAQ from "@/components/FAQ";
import Contact from "@/components/Contact";

export default function Home() {
  return (
    <main className="relative w-full bg-background">
      <Hero />
      <Logo />
      <WhatWeDo />
      <WebResult />
      <Pricing />
      <WhatWeBuild />
      <FAQ />
      <Contact />
    </main>
  );
}
