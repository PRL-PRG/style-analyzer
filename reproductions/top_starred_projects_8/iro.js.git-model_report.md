# Model report for file:///tmp/top-repos-quality-repos-t86o6_ao/iro.js.git HEAD 979b85d5177d621c69edd20f8da5e1ddb0b34ba6

### Dump

```json
{'created_at': '2021-08-29 13:34:10',
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
 'size': '16.2 kB',
 'tags': [],
 'uuid': 'c31ba8bd-e433-4230-a7ef-1094bb1d11ea',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-t86o6_ao/iro.js.git 979b85d5177d621c69edd20f8da5e1ddb0b34ba6

# javascript
14 rules, avg.len. 5.0
## train
PPCR: 0.875401
### report
macro
{'f1-score': 0.7822390551374436,
 'precision': 0.8073983161033372,
 'recall': 0.7631959677638351,
 'support': 8740}
micro
{'f1-score': 0.9554919908466818,
 'precision': 0.9554919908466819,
 'recall': 0.9554919908466819,
 'support': 8740}
weighted
{'f1-score': 0.9537963714888075,
 'precision': 0.9545866256919627,
 'recall': 0.9554919908466819,
 'support': 8740}
### report_full
macro
{'f1-score': 0.722550295137056,
 'precision': 0.8073983161033372,
 'recall': 0.6755844336104305,
 'support': 9984}
micro
{'f1-score': 0.8920102542191839,
 'precision': 0.9554919908466819,
 'recall': 0.8364383012820513,
 'support': 9984}
weighted
{'f1-score': 0.8792314333386805,
 'precision': 0.9432831959777133,
 'recall': 0.8364383012820513,
 'support': 9984}
## test
PPCR: 0.833755
### report
macro
{'f1-score': 0.6939696604019764,
 'precision': 0.7891696066149639,
 'recall': 0.6579609881023353,
 'support': 1976}
micro
{'f1-score': 0.8724696356275303,
 'precision': 0.8724696356275303,
 'recall': 0.8724696356275303,
 'support': 1976}
weighted
{'f1-score': 0.8625819860740239,
 'precision': 0.8710315789297975,
 'recall': 0.8724696356275303,
 'support': 1976}
### report_full
macro
{'f1-score': 0.6468604755477261,
 'precision': 0.7891696066149639,
 'recall': 0.5991503886489663,
 'support': 2370}
micro
{'f1-score': 0.7933732167510353,
 'precision': 0.8724696356275303,
 'recall': 0.7274261603375527,
 'support': 2370}
weighted
{'f1-score': 0.7641302824573355,
 'precision': 0.8503549265869559,
 'recall': 0.7274261603375527,
 'support': 2370}
```

## javascript
### Summary
9 rules, avg.len. 4.9

| | |
|-|-|
|Min support|124|
|Max support|3192|
|Min confidence|0.9339080452919006|
|Max confidence|0.9985835552215576|

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
                     'min_samples_leaf': 95,
                     'min_samples_split': 203,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -2.roles in {MAP}<br>	∧ ^1.roles in {MAP}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 885.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -2.roles not in {MAP}<br>⇒ y = '<br>Confidence: 0.999. Support: 353.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles in {IDENTIFIER} and not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>⇒ y = ∅<br>Confidence: 0.934. Support: 522.` |
| 4 | `  •••start_col ≥ 37<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ -2.roles not in {IDENTIFIER, MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>⇒ y = ⏎<br>Confidence: 0.967. Support: 229.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.998. Support: 213.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {NUMBER}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.944. Support: 135.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.996. Support: 133.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.988. Support: 124.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>⇒ y = ∅<br>Confidence: 0.975. Support: 3192.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.888888888888889, "max_conf": 0.9985835552215576, "max_support": 3192, "min_conf": 0.9339080452919006, "min_support": 124, "num_rules": 9}}
```
</details>
