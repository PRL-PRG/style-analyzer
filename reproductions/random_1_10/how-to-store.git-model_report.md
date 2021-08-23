# Model report for file:///tmp/top-repos-quality-repos-o4ptc0_e/how-to-store.git HEAD 389f5c880ffe63d793ae8cea04cf1abe6a9bf16b

### Dump

```json
{'created_at': '2021-08-20 12:17:47',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.7 kB',
 'tags': [],
 'uuid': 'ff86d13b-54d3-49ba-b3b0-06a381f1b3a3',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-o4ptc0_e/how-to-store.git 389f5c880ffe63d793ae8cea04cf1abe6a9bf16b

# javascript
11 rules, avg.len. 5.8
## train
PPCR: 0.689427
### report
macro
{'f1-score': 0.4492811808814054,
 'precision': 0.4656928297829447,
 'recall': 0.4366223356157449,
 'support': 4069}
micro
{'f1-score': 0.9164413860899484,
 'precision': 0.9164413860899484,
 'recall': 0.9164413860899484,
 'support': 4069}
weighted
{'f1-score': 0.9034975737585006,
 'precision': 0.8955401588573849,
 'recall': 0.9164413860899484,
 'support': 4069}
### report_full
macro
{'f1-score': 0.37291487639078535,
 'precision': 0.4656928297829447,
 'recall': 0.32296579244506013,
 'support': 5902}
micro
{'f1-score': 0.7479691104202187,
 'precision': 0.9164413860899484,
 'recall': 0.6318197221280921,
 'support': 5902}
weighted
{'f1-score': 0.6827836451938418,
 'precision': 0.7874559634753798,
 'recall': 0.6318197221280921,
 'support': 5902}
## test
PPCR: 0.686164
### report
macro
{'f1-score': 0.4260747692811469,
 'precision': 0.44025080152964075,
 'recall': 0.4144166724660879,
 'support': 1091}
micro
{'f1-score': 0.922089825847846,
 'precision': 0.922089825847846,
 'recall': 0.922089825847846,
 'support': 1091}
weighted
{'f1-score': 0.9095525798920111,
 'precision': 0.8987024014609285,
 'recall': 0.922089825847846,
 'support': 1091}
### report_full
macro
{'f1-score': 0.3367418356811236,
 'precision': 0.44025080152964075,
 'recall': 0.28851435041075074,
 'support': 1590}
micro
{'f1-score': 0.7504662439388289,
 'precision': 0.922089825847846,
 'recall': 0.6327044025157232,
 'support': 1590}
weighted
{'f1-score': 0.6839824834641974,
 'precision': 0.808656642201962,
 'recall': 0.6327044025157232,
 'support': 1590}
```

## javascript
### Summary
6 rules, avg.len. 4.5

| | |
|-|-|
|Min support|91|
|Max support|801|
|Min confidence|0.9505494236946106|
|Max confidence|0.9979919791221619|

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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.995. Support: 105.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.991. Support: 801.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = .<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 249.` |
| 4 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {.}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = "<br>Confidence: 0.995. Support: 95.` |
| 5 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 91.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.length ≤ 1<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {VARIABLE}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.955. Support: 100.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.5, "max_conf": 0.9979919791221619, "max_support": 801, "min_conf": 0.9505494236946106, "min_support": 91, "num_rules": 6}}
```
</details>
