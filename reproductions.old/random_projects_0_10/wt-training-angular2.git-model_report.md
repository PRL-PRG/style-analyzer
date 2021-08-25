# Model report for file:///tmp/top-repos-quality-repos-h4rh65t4/wt-training-angular2.git HEAD 303f2d12f8596da821dc63ab70b1b9be4a0afde9

### Dump

```json
{'created_at': '2021-08-23 04:45:01',
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
 'uuid': 'e60e2060-6f3d-4280-907a-aed208d64600',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-h4rh65t4/wt-training-angular2.git 303f2d12f8596da821dc63ab70b1b9be4a0afde9

# javascript
24 rules, avg.len. 4.1
## train
PPCR: 0.586150
### report
macro
{'f1-score': 0.3775063014412638,
 'precision': 0.38056682150055693,
 'recall': 0.3817518362570655,
 'support': 2150}
micro
{'f1-score': 0.8772093023255814,
 'precision': 0.8772093023255814,
 'recall': 0.8772093023255814,
 'support': 2150}
weighted
{'f1-score': 0.8569611436525418,
 'precision': 0.8518023897155678,
 'recall': 0.8772093023255814,
 'support': 2150}
### report_full
macro
{'f1-score': 0.3211698075278185,
 'precision': 0.38056682150055693,
 'recall': 0.2928798185941043,
 'support': 3668}
micro
{'f1-score': 0.6483327603987625,
 'precision': 0.8772093023255814,
 'recall': 0.5141766630316249,
 'support': 3668}
weighted
{'f1-score': 0.5866268541049715,
 'precision': 0.7184462752425073,
 'recall': 0.5141766630316249,
 'support': 3668}
## test
PPCR: 0.633752
### report
macro
{'f1-score': 0.36598358222649185,
 'precision': 0.36878695197160855,
 'recall': 0.37004950495049505,
 'support': 353}
micro
{'f1-score': 0.8583569405099151,
 'precision': 0.8583569405099151,
 'recall': 0.8583569405099151,
 'support': 353}
weighted
{'f1-score': 0.8357544580063535,
 'precision': 0.8258625071782588,
 'recall': 0.8583569405099151,
 'support': 353}
### report_full
macro
{'f1-score': 0.3198131558171872,
 'precision': 0.36878695197160855,
 'recall': 0.2990511744899665,
 'support': 557}
micro
{'f1-score': 0.6659340659340659,
 'precision': 0.8583569405099151,
 'recall': 0.5439856373429084,
 'support': 557}
weighted
{'f1-score': 0.6033643785163304,
 'precision': 0.7115417439594206,
 'recall': 0.5439856373429084,
 'support': 557}
```

## javascript
### Summary
16 rules, avg.len. 3.9

| | |
|-|-|
|Min support|135|
|Max support|640|
|Min confidence|0.9239713549613953|
|Max confidence|0.9976744055747986|

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
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
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
| 1 | `  -1.reserved = :<br>⇒ y = ␣<br>Confidence: 0.962. Support: 275.` |
| 2 | `  -1.reserved not in {:}<br>	∧ -1.roles in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = '<br>Confidence: 0.997. Support: 180.` |
| 3 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.998. Support: 204.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +2.length ≤ 11<br>	∧ ^1.roles in {QUALIFIED} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 154.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.roles not in {STRING}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 151.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.roles not in {MAP, STRING}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 559.` |
| 7 | `  -1.roles in {STRING}<br>⇒ y = '<br>Confidence: 0.998. Support: 215.` |
| 8 | `  -1.reserved = :<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 180.` |
| 9 | `  -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 143.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved not in {:}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 135.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.reserved not in {:}<br>	∧ +2.length ≤ 10<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 154.` |
| 12 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {:}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = '<br>Confidence: 0.997. Support: 178.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 640.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.roles in {STRING}<br>⇒ y = '<br>Confidence: 0.996. Support: 139.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.roles not in {MAP, STRING}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 586.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 590.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.9375, "max_conf": 0.9976744055747986, "max_support": 640, "min_conf": 0.9239713549613953, "min_support": 135, "num_rules": 16}}
```
</details>
