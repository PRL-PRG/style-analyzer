# Model report for file:///tmp/top-repos-quality-repos-7nnx1_qs/nocturn.git HEAD 0add6d272361cb09322fd40c475c29784a144267

### Dump

```json
{'created_at': '2021-08-30 03:07:46',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.11.0-31-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.0 kB',
 'tags': [],
 'uuid': 'cc7eb257-f65b-48a1-820f-d512e172ad2c',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-7nnx1_qs/nocturn.git 0add6d272361cb09322fd40c475c29784a144267

# javascript
17 rules, avg.len. 5.9
## train
PPCR: 0.880260
### report
macro
{'f1-score': 0.6509392700631172,
 'precision': 0.6631037739720911,
 'recall': 0.6408305941248287,
 'support': 13681}
micro
{'f1-score': 0.9323879833345515,
 'precision': 0.9323879833345515,
 'recall': 0.9323879833345515,
 'support': 13681}
weighted
{'f1-score': 0.9242110558990329,
 'precision': 0.9171463592907452,
 'recall': 0.9323879833345515,
 'support': 13681}
### report_full
macro
{'f1-score': 0.623784572032244,
 'precision': 0.6631037739720911,
 'recall': 0.5910723933494787,
 'support': 15542}
micro
{'f1-score': 0.8730109844985114,
 'precision': 0.9323879833345515,
 'recall': 0.820743791017887,
 'support': 15542}
weighted
{'f1-score': 0.8464134152996364,
 'precision': 0.8752491178919851,
 'recall': 0.820743791017887,
 'support': 15542}
## test
PPCR: 0.865723
### report
macro
{'f1-score': 0.6526642348494318,
 'precision': 0.6527456308442537,
 'recall': 0.654864270786325,
 'support': 2018}
micro
{'f1-score': 0.9276511397423192,
 'precision': 0.9276511397423192,
 'recall': 0.9276511397423192,
 'support': 2018}
weighted
{'f1-score': 0.9172213626925758,
 'precision': 0.9081289994577267,
 'recall': 0.9276511397423192,
 'support': 2018}
### report_full
macro
{'f1-score': 0.6144261114909966,
 'precision': 0.6527456308442537,
 'recall': 0.5866542189961137,
 'support': 2331}
micro
{'f1-score': 0.8608875603587031,
 'precision': 0.9276511397423192,
 'recall': 0.803088803088803,
 'support': 2331}
weighted
{'f1-score': 0.8290963614776085,
 'precision': 0.8591704609936741,
 'recall': 0.803088803088803,
 'support': 2331}
```

## javascript
### Summary
12 rules, avg.len. 5.2

| | |
|-|-|
|Min support|90|
|Max support|2312|
|Min confidence|0.9628571271896362|
|Max confidence|0.9988763928413391|

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
                     'min_samples_split': 232,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 2312.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 445.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.972. Support: 90.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 525.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 272.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 154.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 101.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -5.diff_line ≥ 1<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 189.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 122.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 359.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 92.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 1799.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.25, "max_conf": 0.9988763928413391, "max_support": 2312, "min_conf": 0.9628571271896362, "min_support": 90, "num_rules": 12}}
```
</details>
