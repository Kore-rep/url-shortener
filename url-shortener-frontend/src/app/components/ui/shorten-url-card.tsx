'use client'
import { useState } from "react";
import { Button } from "./button";
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "./card";
import { Input } from "./input";
import { Url } from "url";
import { useToast } from "./use-toast";
import { ButtonLoading } from "./button-loading";
import { ToastClose } from "@radix-ui/react-toast";

type CardProps = React.ComponentProps<typeof Card>;

export function ShortenUrlCard(this: any, {className, ...props}: CardProps) {
    const [longUrl, setLongUrl] = useState<string>("");
    const { toast } = useToast();
    const [loading, setLoading] = useState<Boolean>(false);

     const checkUrl = () => {
        console.log("Checking URL");
        if (!longUrl.match(/^http?:/) && longUrl.length) {
            ToastClose
            toast({title: "Editing URL", description: "Automatically added the 'http' protocol to your address"})
            console.log("editing");
            const str = "http://" + longUrl;
            setLongUrl(str)
            return str; 
        }
        return longUrl;
    }

    const handleSubmit = () => {
        setLoading(true);
        checkUrl();
        
        try {
            const url = new URL(checkUrl());
            // TODO: Submit url to backend
        } catch {
            toast({title: "Invalid URL", description: "Your URL did not match specification. Please check it.", variant: "default"})
        } finally {
            setLoading(false);
        }
        
    }


    return (
        <Card className={className} {...props}>
            <CardHeader className="items-center">
                <CardTitle>
                    <div>Shorten URL</div></CardTitle>
            </CardHeader>
            <CardContent>
                <Input type="url" placeholder="https://www.google.com" value={longUrl} onChange={e => setLongUrl(e.target.value)}/>
            </CardContent>
            <CardFooter className="justify-center">
                {loading ? <ButtonLoading /> :<Button variant={"outline"} disabled={longUrl.length < 1} type="button" onClick={handleSubmit} form="urlForm">
                    Generate
                </Button>}
            </CardFooter>
        </Card>
    )
}