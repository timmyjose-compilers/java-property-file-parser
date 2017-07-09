This is a small parser for Java `.properties` files. The main idea of this project is to learn the basics of Parser Combinators, using the Rust `nom` crate as the driving tool. As such, this project does not aim to be 100% compliant with Java `.properties` file specifications. 

The following is the tentative grammar followed by this tool (shown as a pseudo-ABNF grammar):

```
line -> comment | row comment?

comment -> ('#' | '!') TEXT*

row -> TEXT* (':' | '=') TEXT*

TEXT -> [^#!'\n']
```

* Keys with spaces are allowed.

* Unicode values are allowed.

* Trailing whitespace for values are not trimmed (like is done for keys).

* Comments on the same line as key-value pairs are *not* allowed for now.


Some sample files are provided in the `samples` directory for testing our the tool.

## Usage

```
    $ cargo run property-file [property-file]*
```

## Build

```
    $ cargo build --release
```

## Demo

Running the tool in the provided sample files in the `samples` directory gives:

```
Macushla:java-prop-parser z0ltan$ cargo run samples/basic.properties samples/intermediate.properties samples/advanced.properties
   Compiling jpropparser v0.1.0 (file:///Users/z0ltan/Projects/java-prop-parser)
    Finished dev [unoptimized + debuginfo] target(s) in 0.40 secs
     Running `target/debug/jpropparser samples/basic.properties samples/intermediate.properties samples/advanced.properties`
age = 5
breed = Husky
name = Rollo
demeanour = grumpy

language = English
path = c:\\wiki\\templates
tab = \u0009
key\ with\ spaces = This is the value that could be looked up with the key "key with spaces".
website = https://en.wikipedia.org/

appsvr-credcache-life = 7200
appsvr-dbrefresh = appsvr-domain=Default
appsvr-port = 7135
appsvr-servername = PDG1-svr
version = 510
appsvr-username = PDG1/svr
client.serverPort = 7131
pdcert-dn = cn\=PDG1/pdg1,cn\=SecurityDaemons,secAuthority\=Default
pdcert-url = file\:/c\:/tmp/pd/pdadmin.ks
client.source = localSource
appsvr-mode = remote
ppsvr-authzsvrs = PDG1\:7136\:1;
appsvr-listen = false
appsvr-configname = PDG1
appsvr-plcysvrs = PDG1\:7135\:1;
client.doAudit = false
tcd_home = C\:\\PROGRA~1
jar-files = c\:\\program files\\ibm\\java131\\jre\\lib\\ext\\PD.jar
tcd_enabled = true
appsvr-host = pdg1
local_domain = Default
mgmt_domain = Default
pdvar-home = C\:\\PROGRA~1\\Tivoli\\POLICY~1
config_type = full
pd-home = C\:\\PROGRA~1\\Tivoli\\POLICY~1
client.deliveryPolicy = com.tivoli.pd.jaudit.client.AMAuditClientDeliveryPolicy
tivoli_common_dir = C\:\\Program Files\\ibm\\tivoli\\common
java-home = c\:\\program files\\ibm\\java131\\jre
client.serverHost = localhost

```


 


