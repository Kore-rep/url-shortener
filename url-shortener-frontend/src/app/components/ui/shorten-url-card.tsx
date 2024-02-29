'use client'
import { useState } from 'react'
import { Button } from './button'
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from './card'
import { Input } from './input'
import { useToast } from './use-toast'
import { ButtonLoading } from './button-loading'
import { ToastClose } from '@radix-ui/react-toast'

type CardProps = React.ComponentProps<typeof Card>

export function ShortenUrlCard (this: unknown, { className, ...props }: CardProps) {
  const [longUrl, setLongUrl] = useState<string>('')
  const { toast, dismissAllToast } = useToast()
  const [loading, setLoading] = useState<boolean>(false)
  const [btnEffect, setBtnEffect] = useState(false);
  const [inputEffect, setInputEffect] = useState(false);

  const checkUrl = () => {
    console.log('Checking URL')
    if (!longUrl.match(/^http?:/) && longUrl.length) {
      ToastClose
      toast({ title: 'Editing URL', description: "Automatically added the 'http' protocol to your address" })
      console.log('editing')
      const str = 'http://' + longUrl
      setLongUrl(str)
      return str
    }
    return longUrl
  }

  const handleInputChange = (str: string) => {
    setInputEffect(false);
    setLongUrl(str);
  }
  const handleSubmit = () => {
    dismissAllToast();
    setLoading(true)
    checkUrl()

    try {
      const url = new URL(checkUrl())
      // TODO: Submit url to backend
    } catch {
      setBtnEffect(true);
      setInputEffect(true);
      toast({ title: 'Invalid URL', description: 'Your URL did not match specification. Please check it.', variant: 'destructive' })
    } finally {
      setLoading(false)
    }
  }

  return (
        <Card className={className} {...props}>
            <CardHeader className="items-center">
                <CardTitle>
                    <div>Shorten URL</div></CardTitle>
            </CardHeader>
            <CardContent>
                <Input className={`${inputEffect ? "ease-out duration-invalid-timing border-[#811d1d]": ""}`} type="url" placeholder="https://www.google.com" value={longUrl} onChange={e => { handleInputChange(e.target.value) }}/>
            </CardContent>
            <CardFooter className="justify-center">
                {loading
                  ? <ButtonLoading />
                  : <Button className={`${btnEffect && "animate-shake"}`}variant={'outline'} disabled={longUrl.length < 1} type="button" onClick={handleSubmit} onAnimationEnd={() => setBtnEffect(false)} form="urlForm">
                    Generate
                </Button>}
            </CardFooter>
        </Card>
  )
}
