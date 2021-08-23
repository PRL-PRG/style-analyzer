# Model report for file:///tmp/top-repos-quality-repos-n5znpwc5/secure-telemetry-server-deprecated.git HEAD ebde174bc64739e41d19a6d1627359af8c2e53a2

### Dump

```json
{'created_at': '2021-08-21 18:25:38',
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
 'size': '18.2 kB',
 'tags': [],
 'uuid': 'cf3133ab-a573-4550-9af5-a739fa0e2502',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-n5znpwc5/secure-telemetry-server-deprecated.git ebde174bc64739e41d19a6d1627359af8c2e53a2

# javascript
14 rules, avg.len. 5.5
## train
PPCR: 0.840771
### report
macro
{'f1-score': 0.35163778874352786,
 'precision': 0.3627565396072296,
 'recall': 0.34337856557966906,
 'support': 12435}
micro
{'f1-score': 0.948854041013269,
 'precision': 0.948854041013269,
 'recall': 0.948854041013269,
 'support': 12435}
weighted
{'f1-score': 0.938002565589848,
 'precision': 0.929018773636841,
 'recall': 0.948854041013269,
 'support': 12435}
### report_full
macro
{'f1-score': 0.2800729065928116,
 'precision': 0.3627565396072296,
 'recall': 0.2531037916987931,
 'support': 14790}
micro
{'f1-score': 0.8667768595041321,
 'precision': 0.948854041013269,
 'recall': 0.7977687626774848,
 'support': 14790}
weighted
{'f1-score': 0.8080330948679181,
 'precision': 0.8497573450081124,
 'recall': 0.7977687626774848,
 'support': 14790}
## test
PPCR: 0.903743
### report
macro
{'f1-score': 0.25813057194861766,
 'precision': 0.26062266778056253,
 'recall': 0.2774828857293312,
 'support': 676}
micro
{'f1-score': 0.8387573964497042,
 'precision': 0.8387573964497042,
 'recall': 0.8387573964497042,
 'support': 676}
weighted
{'f1-score': 0.8070785528891157,
 'precision': 0.7830107484358496,
 'recall': 0.8387573964497042,
 'support': 676}
### report_full
macro
{'f1-score': 0.24691688862428057,
 'precision': 0.26062266778056253,
 'recall': 0.2472068857168507,
 'support': 748}
micro
{'f1-score': 0.7963483146067415,
 'precision': 0.8387573964497042,
 'recall': 0.7580213903743316,
 'support': 748}
weighted
{'f1-score': 0.7414067356588291,
 'precision': 0.7323779889414564,
 'recall': 0.7580213903743316,
 'support': 748}
```

## javascript
### Summary
11 rules, avg.len. 4.6

| | |
|-|-|
|Min support|104|
|Max support|2636|
|Min confidence|0.9236111044883728|
|Max confidence|0.9976303577423096|

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
| 1 | `  -1.reserved = (<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 107.` |
| 2 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 1868.` |
| 3 | `  ^1.roles in {QUALIFIED} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 2636.` |
| 4 | `  -1.internal_type = Identifier<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 104.` |
| 5 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 1515.` |
| 6 | `  -1.internal_type not in {Identifier}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 504.` |
| 7 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 544.` |
| 8 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 258.` |
| 9 | `  -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 211.` |
| 10 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.944. Support: 243.` |
| 11 | `  -1.internal_type not in {CommentLine, Identifier, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 137.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.636363636363637, "max_conf": 0.9976303577423096, "max_support": 2636, "min_conf": 0.9236111044883728, "min_support": 104, "num_rules": 11}}
```
</details>
