# Model report for file:///tmp/top-repos-quality-repos-mhwba1tl/guys-night-out-big2.git HEAD dcc31501fa91e7e7c1159559399f18f8dd5389a6

### Dump

```json
{'created_at': '2021-08-22 11:32:33',
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
 'size': '15.7 kB',
 'tags': [],
 'uuid': 'b436037d-b13b-40d8-b7c4-0015e6dd6a82',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-mhwba1tl/guys-night-out-big2.git dcc31501fa91e7e7c1159559399f18f8dd5389a6

# javascript
12 rules, avg.len. 6.1
## train
PPCR: 0.800432
### report
macro
{'f1-score': 0.6443846692789308,
 'precision': 0.6634339353899835,
 'recall': 0.6352687120422622,
 'support': 5555}
micro
{'f1-score': 0.9404140414041404,
 'precision': 0.9404140414041404,
 'recall': 0.9404140414041404,
 'support': 5555}
weighted
{'f1-score': 0.9389360797653123,
 'precision': 0.9414066515392264,
 'recall': 0.9404140414041404,
 'support': 5555}
### report_full
macro
{'f1-score': 0.5563600071531727,
 'precision': 0.6634339353899835,
 'recall': 0.4978724861169406,
 'support': 6940}
micro
{'f1-score': 0.8361744697879151,
 'precision': 0.9404140414041404,
 'recall': 0.7527377521613833,
 'support': 6940}
weighted
{'f1-score': 0.8097908017542621,
 'precision': 0.8893175303926769,
 'recall': 0.7527377521613833,
 'support': 6940}
## test
PPCR: 0.760000
### report
macro
{'f1-score': 0.49845141308555935,
 'precision': 0.5051948051948052,
 'recall': 0.5102040816326531,
 'support': 38}
micro
{'f1-score': 0.8421052631578947,
 'precision': 0.8421052631578947,
 'recall': 0.8421052631578947,
 'support': 38}
weighted
{'f1-score': 0.8346526886321494,
 'precision': 0.8514354066985647,
 'recall': 0.8421052631578947,
 'support': 38}
### report_full
macro
{'f1-score': 0.44860565790798346,
 'precision': 0.5051948051948052,
 'recall': 0.4169960474308301,
 'support': 50}
micro
{'f1-score': 0.7272727272727272,
 'precision': 0.8421052631578947,
 'recall': 0.64,
 'support': 50}
weighted
{'f1-score': 0.6984496124031009,
 'precision': 0.794,
 'recall': 0.64,
 'support': 50}
```

## javascript
### Summary
8 rules, avg.len. 5.0

| | |
|-|-|
|Min support|90|
|Max support|1202|
|Min confidence|0.9385964870452881|
|Max confidence|0.9975961446762085|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.992. Support: 1202.` |
| 2 | `  +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 280.` |
| 3 | `  +1.roles in {RIGHT}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 171.` |
| 4 | `  -1.roles in {EXPRESSION, STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.996. Support: 120.` |
| 5 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 832.` |
| 6 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 208.` |
| 7 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 94.` |
| 8 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.950. Support: 90.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.0, "max_conf": 0.9975961446762085, "max_support": 1202, "min_conf": 0.9385964870452881, "min_support": 90, "num_rules": 8}}
```
</details>
