# Model report for file:///tmp/top-repos-quality-repos-4pnbbv7r/node-react-ecommerce.git HEAD 51896ab498b92f6ad81df5fdbc9e05f43c330b9c

### Dump

```json
{'created_at': '2021-08-30 08:43:18',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '17.0 kB',
 'tags': [],
 'uuid': 'd3151cf6-bcbd-42a9-9b6b-4f1ec3ccf7d1',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-4pnbbv7r/node-react-ecommerce.git 51896ab498b92f6ad81df5fdbc9e05f43c330b9c

# javascript
17 rules, avg.len. 5.8
## train
PPCR: 0.871011
### report
macro
{'f1-score': 0.626985359645297,
 'precision': 0.6895003754920745,
 'recall': 0.6013272419894755,
 'support': 12884}
micro
{'f1-score': 0.9271189071716858,
 'precision': 0.9271189071716858,
 'recall': 0.9271189071716858,
 'support': 12884}
weighted
{'f1-score': 0.9206709040591032,
 'precision': 0.9231524005303665,
 'recall': 0.9271189071716858,
 'support': 12884}
### report_full
macro
{'f1-score': 0.5575671578980752,
 'precision': 0.6895003754920745,
 'recall': 0.5097076709901562,
 'support': 14792}
micro
{'f1-score': 0.8632027749674809,
 'precision': 0.9271189071716858,
 'recall': 0.8075310978907517,
 'support': 14792}
weighted
{'f1-score': 0.8379494844538473,
 'precision': 0.9000776916365133,
 'recall': 0.8075310978907517,
 'support': 14792}
## test
PPCR: 0.896431
### report
macro
{'f1-score': 0.5565611889589794,
 'precision': 0.6858442048843613,
 'recall': 0.5329180136769965,
 'support': 3064}
micro
{'f1-score': 0.914490861618799,
 'precision': 0.914490861618799,
 'recall': 0.914490861618799,
 'support': 3064}
weighted
{'f1-score': 0.9068669761274429,
 'precision': 0.9103238459986508,
 'recall': 0.914490861618799,
 'support': 3064}
### report_full
macro
{'f1-score': 0.5001605966040206,
 'precision': 0.6858442048843613,
 'recall': 0.4564198833233163,
 'support': 3418}
micro
{'f1-score': 0.8645479790188213,
 'precision': 0.914490861618799,
 'recall': 0.8197776477472206,
 'support': 3418}
weighted
{'f1-score': 0.8432255568991865,
 'precision': 0.8954903001317308,
 'recall': 0.8197776477472206,
 'support': 3418}
```

## javascript
### Summary
10 rules, avg.len. 5.4

| | |
|-|-|
|Min support|97|
|Max support|2378|
|Min confidence|0.948888897895813|
|Max confidence|0.9990741014480591|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'max_depth': 10,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.roles not in {LITERAL}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.984. Support: 2378.` |
| 2 | `  +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 540.` |
| 3 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.949. Support: 225.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.997. Support: 144.` |
| 5 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ +1.roles in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {INITIALIZATION}<br>⇒ y = ⏎<br>Confidence: 0.974. Support: 97.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -3.reserved = const<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 149.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -3.reserved not in {const}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 2313.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 309.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 241.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 126.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.4, "max_conf": 0.9990741014480591, "max_support": 2378, "min_conf": 0.948888897895813, "min_support": 97, "num_rules": 10}}
```
</details>
