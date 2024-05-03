# Apple Music Lyrics

## About This Project

Get lyrics for Apple Music

## Framework Used
- [dotenv](https://github.com/dotenv-rs/dotenv) 0.15
- [fancy-regex](https://github.com/fancy-regex/fancy-regex) 0.13 
- [quick-xml](https://github.com/tafia/quick-xml) 0.31
- [reqwest](https://github.com/seanmonstar/reqwest) 0.12.2 
- [serde](https://github.com/serde-rs/serde) 1.0.197
- [serde_json](https://github.com/serde-rs/json) 1.0.115
- [tokio](https://github.com/tokio-rs/tokio) 1.36

## Usage

Run the command to start the simulation

```
cargo run
```

If you are running it for the first time, you will be prompted to enter your Apple Music user token. The token will be saved in the .env file, giving you the flexibility to edit it at any time.

## How To Find User Token?

1. Log in to your Apple ID on the Apple Music Web platform.
2. Access the Network Inspector within your browser.
3. Reload the page.
4. Locate the Info file.
5. The user token can be found in the `Media-User-Token` header.

## Response JSON File Structure

```
{
    "lines": [
        {
            "begin": "string",
            "end": "string",
            "words": [
                {
                    "begin": "string",
                    "end": "string",
                    "text": "string"
                }
            ],
            "background": [
                "words": [
                    {
                        "begin": "string",
                        "end": "string",
                        "text": "string"
                    }
                ],
            ]
        }
    ]
}
```


