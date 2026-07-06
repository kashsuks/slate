# Slate

Slate is a selfhosted suite of project management tools. Oh also its FOSS!

## Installation

Currently under dev, but once ready for beta testing will be deployed on Cargo and static builds on Github Releases

## Usage

If using docker, first ensure you have the CLI and then run the following:

1. Build the image

```bash
docker build -t slate .
```
2. Run the image
```bash
docker run -d \
  -p 3000:3000 \
  -v /your/data/path:/data \
  --name slate \
  slate
```
## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)
