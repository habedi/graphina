## Test Data

This directory contains graph datasets used for testing purposes.

### Graph Datasets

The following table lists the graph datasets used for testing various graph algorithms.
Each dataset is described by the following attributes:

- **Index**: A unique identifier for the dataset.
- **Dataset Name**: The name of the dataset.
- **Network Type**: The type of network represented by the dataset.
- **\#Nodes**: The number of nodes in the network.
- **\#Edges**: The number of edges in the network.
- **Directed**: A flag indicating whether the network is directed or undirected.
- **Weighted**: A flag indicating whether the network is weighted or unweighted.

| Index | Dataset Name             | Network Type     | \#Nodes | \#Edges   | Directed | Weighted | Download Link                                                               |
|-------|--------------------------|------------------|---------|-----------|----------|----------|-----------------------------------------------------------------------------|
| 1     | Zachary's Karate Club    | social network   | 34      | 156       | No       | No       | [Link](http://vlado.fmf.uni-lj.si/pub/networks/data/Ucinet/UciData.htm)     |
| 2     | Cora                     | citation network | 2,708   | 10,556    | No       | No       | [Link](https://linqs.org/datasets/#cora)                                    |
| 3     | CiteSeer                 | citation network | 3,327   | 9,104     | No       | No       | [Link](https://linqs.org/datasets/#citeseer-doc-classification)             |
| 4     | PubMed                   | citation network | 19,717  | 88,648    | No       | Yes      | [Link](https://linqs.org/datasets/#pubmed-diabetes)                         |
| 5     | Facebook Page-Page       | social network   | 22,470  | 171,002   | No       | No       | [Link](http://snap.stanford.edu/data/facebook-large-page-page-network.html) |
| 6     | Wikipedia Chameleon      | article network  | 2,277   | 31,421    | No       | No       | [Link](https://snap.stanford.edu/data/wikipedia-article-networks.html)      |
| 7     | Wikipedia Crocodile      | article network  | 11,631  | 170,918   | No       | No       | [Link](https://snap.stanford.edu/data/wikipedia-article-networks.html)      |
| 8     | Wikipedia Squirrel       | article network  | 5,201   | 198,493   | No       | No       | [Link](https://snap.stanford.edu/data/wikipedia-article-networks.html)      |
| 9     | DBLP Citation Network    | citation network | 317,080 | 1,049,866 | No       | No       | [Link](http://snap.stanford.edu/data/com-DBLP.html)                         |
| 10    | Amazon Products          | co-purchasing    | 548,552 | 1,851,744 | No       | Yes      | [Link](http://snap.stanford.edu/data/amazon-meta.html)                      |
| 11    | Stanford Web Graph       | web graph        | 281,903 | 2,312,497 | Yes      | No       | [Link](http://snap.stanford.edu/data/web-Stanford.html)                     |
| 12    | Google Web Graph         | web graph        | 875,713 | 5,105,039 | Yes      | No       | [Link](http://snap.stanford.edu/data/web-Google.html)                       |
| 13    | Reddit Hyperlink Network | social network   | 55,863  | 858,490   | Yes      | Yes      | [Link](http://snap.stanford.edu/data/soc-RedditHyperlinks.html)             |
