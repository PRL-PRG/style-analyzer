# Model report for file:///tmp/top-repos-quality-repos-613ywc0_/prettier-eslint.git HEAD c62769ef430749fb9650c7e1d05f5cf7442a48a2

### Dump

```json
{'created_at': '2021-08-29 22:46:42',
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
 'size': '15.9 kB',
 'tags': [],
 'uuid': '1d8cba56-3141-43b2-a5d0-8c468ba82161',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-613ywc0_/prettier-eslint.git c62769ef430749fb9650c7e1d05f5cf7442a48a2

# javascript
11 rules, avg.len. 4.8
## train
PPCR: 0.693361
### report
macro
{'f1-score': 0.746236585789835,
 'precision': 0.7823198826713637,
 'recall': 0.7317130979697296,
 'support': 4303}
micro
{'f1-score': 0.9144782709737392,
 'precision': 0.9144782709737392,
 'recall': 0.9144782709737392,
 'support': 4303}
weighted
{'f1-score': 0.9054889171452252,
 'precision': 0.9162363891253473,
 'recall': 0.9144782709737392,
 'support': 4303}
### report_full
macro
{'f1-score': 0.5942332862239585,
 'precision': 0.7823198826713637,
 'recall': 0.5333047780033253,
 'support': 6206}
micro
{'f1-score': 0.7488819107431726,
 'precision': 0.9144782709737392,
 'recall': 0.6340638092168869,
 'support': 6206}
weighted
{'f1-score': 0.684344188223807,
 'precision': 0.9006103385012487,
 'recall': 0.6340638092168869,
 'support': 6206}
## test
PPCR: 0.691521
### report
macro
{'f1-score': 0.5435185427383987,
 'precision': 0.6158687493145326,
 'recall': 0.6658234058838538,
 'support': 1150}
micro
{'f1-score': 0.7643478260869564,
 'precision': 0.7643478260869565,
 'recall': 0.7643478260869565,
 'support': 1150}
weighted
{'f1-score': 0.6975536159023906,
 'precision': 0.8637426764712935,
 'recall': 0.7643478260869565,
 'support': 1150}
### report_full
macro
{'f1-score': 0.47254210042709444,
 'precision': 0.6158687493145326,
 'recall': 0.511322984971173,
 'support': 1663}
micro
{'f1-score': 0.6249555634553857,
 'precision': 0.7643478260869565,
 'recall': 0.5285628382441371,
 'support': 1663}
weighted
{'f1-score': 0.5243353673249382,
 'precision': 0.8702659494501795,
 'recall': 0.5285628382441371,
 'support': 1663}
```

## javascript
### Summary
7 rules, avg.len. 4.0

| | |
|-|-|
|Min support|94|
|Max support|1171|
|Min confidence|0.9208333492279053|
|Max confidence|0.9980079531669617|

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
| 1 | `  -1.internal_type = Identifier<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.996. Support: 117.` |
| 2 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 1171.` |
| 3 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.998. Support: 251.` |
| 4 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.997. Support: 151.` |
| 5 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {KEY}<br>⇒ y = ⏎<br>Confidence: 0.921. Support: 120.` |
| 6 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 181.` |
| 7 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 94.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.0, "max_conf": 0.9980079531669617, "max_support": 1171, "min_conf": 0.9208333492279053, "min_support": 94, "num_rules": 7}}
```
</details>
