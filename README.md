# Structnmap

This is a tool for parsing nmap xml and structing it to files by service name.

**USAGE:** <br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`$ structnmap <xml> <output>`

**FLAGS:** <br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`-h, --help` - Prints help information <br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`-V, --version` - Prints version information <br/>

**ARGS:** <br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`<xml>` - Nmap xml file path <br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`<output>` - Output directory

**EXAMPLE:** <br/>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`$ structnmap nmap.xml output` <br/>
You can see an output sample in example directory in project repository

**IN-CODE USAGE EXAMPLE:** <br/>

```rust
use structnmap::{Data, Error};

fn main() -> Result<(), Error> {
    let test = Data::build("nmap.xml")?;
    test.generate("output")?;
    Ok(())
}
```
