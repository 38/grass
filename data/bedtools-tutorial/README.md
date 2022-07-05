# PyGRASS implementation of bedtools tutorial

This is a set of examples on how to use PyGRASS to implement the bedtools tutorial.
The original tutorial can be found at the [bedtools tutorial](http://quinlanlab.org/tutorials/bedtools/bedtools.html).

The tutorial shows how to use PyGRASS to implement the equivalent operations.

## Preparation

In order to make the tutorial actually work, you need a few preparation steps:

- Download the data used in the tutorial. This can be done by running the script `get-data.sh` under this directory.

- Configure the environment variables to use the local build of PyGRASS. This can be done by running the following command:

```bash
source build-and-env.sh
```

After these steps, you should be able to run the tutorial.

Each example is corresponding to one example in the bedtools tutorial.