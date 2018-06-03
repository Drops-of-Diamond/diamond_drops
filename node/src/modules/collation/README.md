# collation quick start

## run a local submodule test
We want to test a local submodule before testing a global module.

For example: collation_index.rs
``` 
cd diamond_drops/node/src/modules/collation
cargo test collation_index
```

## run a top level module compilation & test
We want to test a top level module.

First clean up the previous build artifacts:
``` 
cargo clean
```

Now compile and test the node collation
``` 
cd diamond_drops/node
cargo test node
```


## You can also test a middle module:
``` 
cd diamond_drops/node/src/modules
cargo clean
cargo test collation
```
