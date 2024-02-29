import Image from "next/image";
import { BackgroundBeams } from "./components/ui/background-beams";
import { Boxes } from "./components/ui/background-boxes";

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <div><text>Hello World</text></div>
      <BackgroundBeams />
     
    </main>
  );
}
