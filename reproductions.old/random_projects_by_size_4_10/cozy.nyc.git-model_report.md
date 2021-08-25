# Model report for file:///tmp/top-repos-quality-repos-lk1woi3x/cozy.nyc.git HEAD 4cfa03fd4762d59a89e10b244e67b1359da3e7a9

### Dump

```json
{'created_at': '2021-08-21 21:11:40',
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
 'uuid': 'd2f4ce99-bda1-4872-9665-5d7211736d59',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-lk1woi3x/cozy.nyc.git 4cfa03fd4762d59a89e10b244e67b1359da3e7a9

# javascript
16 rules, avg.len. 6.1
## train
PPCR: 0.712526
### report
macro
{'f1-score': 0.5937886960110165,
 'precision': 0.5900496633762041,
 'recall': 0.5978810360752239,
 'support': 9659}
micro
{'f1-score': 0.9527901439072368,
 'precision': 0.9527901439072368,
 'recall': 0.9527901439072368,
 'support': 9659}
weighted
{'f1-score': 0.9460951081545037,
 'precision': 0.9396549173233071,
 'recall': 0.9527901439072368,
 'support': 9659}
### report_full
macro
{'f1-score': 0.5088036801225537,
 'precision': 0.5900496633762041,
 'recall': 0.45754888581193615,
 'support': 13556}
micro
{'f1-score': 0.7928494507861298,
 'precision': 0.9527901439072368,
 'recall': 0.6788875774564769,
 'support': 13556}
weighted
{'f1-score': 0.7550438771536184,
 'precision': 0.8602303761714525,
 'recall': 0.6788875774564769,
 'support': 13556}
## test
PPCR: 0.690276
### report
macro
{'f1-score': 0.5867690203165302,
 'precision': 0.5874523699328844,
 'recall': 0.5865168198131634,
 'support': 2378}
micro
{'f1-score': 0.9491169049621531,
 'precision': 0.9491169049621531,
 'recall': 0.9491169049621531,
 'support': 2378}
weighted
{'f1-score': 0.9405140427206184,
 'precision': 0.9325137012028634,
 'recall': 0.9491169049621531,
 'support': 2378}
### report_full
macro
{'f1-score': 0.48931435100891985,
 'precision': 0.5874523699328844,
 'recall': 0.437322066568159,
 'support': 3445}
micro
{'f1-score': 0.7752017860209515,
 'precision': 0.9491169049621531,
 'recall': 0.6551523947750363,
 'support': 3445}
weighted
{'f1-score': 0.7326250218688964,
 'precision': 0.8491910401873781,
 'recall': 0.6551523947750363,
 'support': 3445}
```

## javascript
### Summary
11 rules, avg.len. 5.3

| | |
|-|-|
|Min support|101|
|Max support|1883|
|Min confidence|0.9768985509872437|
|Max confidence|0.9989429116249084|

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
                     'min_samples_split': 240,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type = JSXIdentifier<br>⇒ y = "<br>Confidence: 0.995. Support: 101.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type not in {JSXIdentifier}<br>⇒ y = '<br>Confidence: 0.999. Support: 468.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = .<br>	∧ +1.length ≥ 2<br>⇒ y = ∅<br>Confidence: 0.999. Support: 473.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {.}<br>	∧ +1.length ≥ 2<br>⇒ y = '<br>Confidence: 0.998. Support: 279.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {.}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.992. Support: 315.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = Identifier<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 245.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ., ;, {}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.roles not in {KEY}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = "<br>Confidence: 0.995. Support: 110.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 198.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 112.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 126.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ +3.length ≥ 1<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 1883.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.2727272727272725, "max_conf": 0.9989429116249084, "max_support": 1883, "min_conf": 0.9768985509872437, "min_support": 101, "num_rules": 11}}
```
</details>
