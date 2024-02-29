import Image from "next/image";
import { BackgroundBeams } from "./components/ui/background-beams";
import { Boxes } from "./components/ui/background-boxes";
import { ShortenUrlCard } from "./components/ui/shorten-url-card";

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div className="z-10">
        <ShortenUrlCard /> 
      </div>
      <BackgroundBeams />
      {/* <Boxes /> */}
      
    </main>
  );
}
