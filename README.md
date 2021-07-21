# Overview

The `outcome` crate provides several augmentations of the `Result` type found
in the Rust standard library. It is *first and foremost* an experiment in
expressing more granular error handling contracts. It is secondly written for
library authors.

The current package name on [crates.io][1] was generated via python 3's `uuid`:

```python
from uuid import *

print(uuid5(uuid5(NAMESPACE_DNS, "occult.work"), "outcome"))
```

Detailed information on `outcome` can be found in the crate [documentation][2].

[1]: https://crates.io/crates/outcome-46f94afc-026f-5511-9d7e-7d1fd495fb5c
[2]: #
