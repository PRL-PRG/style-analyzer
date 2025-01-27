# Model report for file:///tmp/top-repos-quality-repos-g97dacqi/asar.git HEAD b0415c8ef29c22aeedfc5b4e179ef8969d5c43a6

### Dump

```json
{'created_at': '2021-08-29 14:45:52',
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
 'size': '15.1 kB',
 'tags': [],
 'uuid': '68b82da4-967b-47ff-ab52-b1aec23c210b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-g97dacqi/asar.git b0415c8ef29c22aeedfc5b4e179ef8969d5c43a6

# javascript
8 rules, avg.len. 5.2
## train
PPCR: 0.734789
### report
macro
{'f1-score': 0.7748383471470145,
 'precision': 0.7732645753007233,
 'recall': 0.7800750565721488,
 'support': 3164}
micro
{'f1-score': 0.9345764854614412,
 'precision': 0.9345764854614412,
 'recall': 0.9345764854614412,
 'support': 3164}
weighted
{'f1-score': 0.9327802356796726,
 'precision': 0.9321005676834538,
 'recall': 0.9345764854614412,
 'support': 3164}
### report_full
macro
{'f1-score': 0.6411780633596732,
 'precision': 0.7732645753007233,
 'recall': 0.5745993883716228,
 'support': 4306}
micro
{'f1-score': 0.7917001338688087,
 'precision': 0.9345764854614412,
 'recall': 0.6867162099396191,
 'support': 4306}
weighted
{'f1-score': 0.7710142934174254,
 'precision': 0.9100238576347903,
 'recall': 0.6867162099396191,
 'support': 4306}
## test
PPCR: 0.732759
### report
macro
{'f1-score': 0.6272916193560195,
 'precision': 0.7159759698834561,
 'recall': 0.603957485694187,
 'support': 680}
micro
{'f1-score': 0.8485294117647059,
 'precision': 0.8485294117647059,
 'recall': 0.8485294117647059,
 'support': 680}
weighted
{'f1-score': 0.8365179890137209,
 'precision': 0.8448746374995524,
 'recall': 0.8485294117647059,
 'support': 680}
### report_full
macro
{'f1-score': 0.501760821633653,
 'precision': 0.7159759698834561,
 'recall': 0.4576572279006777,
 'support': 928}
micro
{'f1-score': 0.7176616915422885,
 'precision': 0.8485294117647059,
 'recall': 0.6217672413793104,
 'support': 928}
weighted
{'f1-score': 0.6779745919486324,
 'precision': 0.8309984218430447,
 'recall': 0.6217672413793104,
 'support': 928}
```

## javascript
### Summary
5 rules, avg.len. 4.2

| | |
|-|-|
|Min support|101|
|Max support|545|
|Min confidence|0.98046875|
|Max confidence|0.9981203079223633|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.986. Support: 545.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.997. Support: 184.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 266.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {CALL}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 101.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {SCOPE} and not in {CALL}<br>⇒ y = ⏎<br>Confidence: 0.980. Support: 128.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.2, "max_conf": 0.9981203079223633, "max_support": 545, "min_conf": 0.98046875, "min_support": 101, "num_rules": 5}}
```
</details>
