# URL shortener API

## Design

The API exposes 2 endpoints.
/url
 - /{key}
   - GET: Get the redirect for a URL key
     - Request to https://shorty.com/12345
     - Response: {}
 - /shorten
   - POST: Post a URL to be shortened.
     - Request: {"url": "https://www.google.com"}
     - Response: {"key": "12345", "long_url": "https://www.google.com", "short_url": "https://shorty.com/12345}


## Technology
The API is hosted on Oracle cloud and uses <insert persistent storage here>