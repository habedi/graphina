## Test Datasets

You can download the graph datasets used in the tests, examples, and benchmarks from Hugging Face (link below):

- [habedi/graphina-graphs](https://huggingface.co/datasets/habedi/graphina-graphs)

Make sure that the downloaded datasets (the `txt` files) are stored in the `tests/testdata/graphina-graphs`
path (realtive to the root of the repository).

### Using Hugging Face CLI Client

You can use [huggingface-cli](https://huggingface.co/docs/huggingface_hub/en/guides/cli) to download the datasets:

```shell
huggingface-cli download habedi/graphina-graphs --repo-type dataset --local-dir graphina-graphs
```
