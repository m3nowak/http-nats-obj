# http-nats-obj

A simple HTTP server for NATS JetStream Object storage, written in Rust.

## Usage
You'll need credential file with appropriate permissions. Also, object bucket must be already created.
```bash
#upload files, this will remove all files already existing
http-nats-obj -c obj.creds --nats localhost --bucket website upload -f --dir upload-dir
#serve the website
http-nats-obj -c obj.creds --nats localhost --bucket website serve
```

## Notes
This is an extremely basic demo missing features include:
- caching
- uploading without clearing everything
- auth methods beside creds
