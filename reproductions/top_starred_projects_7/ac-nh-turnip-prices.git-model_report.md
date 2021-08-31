# Model report for file:///tmp/top-repos-quality-repos-f8kwghfu/ac-nh-turnip-prices.git HEAD 8581f153ca60ba6b8f31791a4eebc40dc3f4afbd

### Dump

```json
{'created_at': '2021-08-30 17:24:40',
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
 'size': '16.4 kB',
 'tags': [],
 'uuid': '79e9576f-ba14-4853-8a3b-b11611a2f834',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-f8kwghfu/ac-nh-turnip-prices.git 8581f153ca60ba6b8f31791a4eebc40dc3f4afbd

# javascript
11 rules, avg.len. 5.2
## train
PPCR: 0.870133
### report
macro
{'f1-score': 0.47015483313325607,
 'precision': 0.46931566642040906,
 'recall': 0.4726636912977347,
 'support': 7196}
micro
{'f1-score': 0.9205113952195664,
 'precision': 0.9205113952195664,
 'recall': 0.9205113952195664,
 'support': 7196}
weighted
{'f1-score': 0.9071081120850173,
 'precision': 0.8977986301219505,
 'recall': 0.9205113952195664,
 'support': 7196}
### report_full
macro
{'f1-score': 0.4649653422739105,
 'precision': 0.46931566642040906,
 'recall': 0.4617817277038183,
 'support': 8270}
micro
{'f1-score': 0.8565886460623303,
 'precision': 0.9205113952195664,
 'recall': 0.8009673518742443,
 'support': 8270}
weighted
{'f1-score': 0.8030243327333463,
 'precision': 0.8077447637255898,
 'recall': 0.8009673518742443,
 'support': 8270}
## test
PPCR: 0.805369
### report
macro
{'f1-score': 0.4667192833819819,
 'precision': 0.4597918096545413,
 'recall': 0.4784691897137312,
 'support': 360}
micro
{'f1-score': 0.8833333333333333,
 'precision': 0.8833333333333333,
 'recall': 0.8833333333333333,
 'support': 360}
weighted
{'f1-score': 0.8517683717024797,
 'precision': 0.8269859427031698,
 'recall': 0.8833333333333333,
 'support': 360}
### report_full
macro
{'f1-score': 0.44918192119469524,
 'precision': 0.4597918096545413,
 'recall': 0.440666349528014,
 'support': 447}
micro
{'f1-score': 0.7881040892193308,
 'precision': 0.8833333333333333,
 'recall': 0.7114093959731543,
 'support': 447}
weighted
{'f1-score': 0.7261464281758717,
 'precision': 0.7440231708172176,
 'recall': 0.7114093959731543,
 'support': 447}
```

## javascript
### Summary
8 rules, avg.len. 4.8

| | |
|-|-|
|Min support|104|
|Max support|1617|
|Min confidence|0.9644549489021301|
|Max confidence|0.998106062412262|

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
                     'min_samples_leaf': 101,
                     'min_samples_split': 239,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 171.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 1617.` |
| 3 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.976. Support: 228.` |
| 4 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.964. Support: 211.` |
| 5 | `  -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {QUALIFIED} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 502.` |
| 6 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 264.` |
| 7 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 114.` |
| 8 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 104.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.75, "max_conf": 0.998106062412262, "max_support": 1617, "min_conf": 0.9644549489021301, "min_support": 104, "num_rules": 8}}
```
</details>
