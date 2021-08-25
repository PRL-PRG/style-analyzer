# Model report for file:///tmp/top-repos-quality-repos-jcwyl908/espaces.git HEAD 3f73fd3a10e27d6aac7307f120e0616f02b84639

### Dump

```json
{'created_at': '2021-08-22 00:49:07',
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
 'size': '18.3 kB',
 'tags': [],
 'uuid': '542455ee-6e58-48f4-9d1f-e544aa58f115',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-jcwyl908/espaces.git 3f73fd3a10e27d6aac7307f120e0616f02b84639

# javascript
11 rules, avg.len. 5.0
## train
PPCR: 0.678649
### report
macro
{'f1-score': 0.28324389359166774,
 'precision': 0.2859117651216022,
 'recall': 0.28103044377746694,
 'support': 8420}
micro
{'f1-score': 0.9704275534441805,
 'precision': 0.9704275534441805,
 'recall': 0.9704275534441805,
 'support': 8420}
weighted
{'f1-score': 0.9649471853311578,
 'precision': 0.9596666854846558,
 'recall': 0.9704275534441805,
 'support': 8420}
### report_full
macro
{'f1-score': 0.19391755249803183,
 'precision': 0.2859117651216022,
 'recall': 0.16042224725808765,
 'support': 12407}
micro
{'f1-score': 0.7846545349786335,
 'precision': 0.9704275534441805,
 'recall': 0.6585798339646973,
 'support': 12407}
weighted
{'f1-score': 0.7263283023611539,
 'precision': 0.8573989524141271,
 'recall': 0.6585798339646973,
 'support': 12407}
## test
PPCR: 0.696373
### report
macro
{'f1-score': 0.2508743421337184,
 'precision': 0.2899058219178082,
 'recall': 0.23594597480247703,
 'support': 1344}
micro
{'f1-score': 0.9709821428571429,
 'precision': 0.9709821428571429,
 'recall': 0.9709821428571429,
 'support': 1344}
weighted
{'f1-score': 0.9675471929592537,
 'precision': 0.9663939553775277,
 'recall': 0.9709821428571429,
 'support': 1344}
### report_full
macro
{'f1-score': 0.16832856920619238,
 'precision': 0.2899058219178082,
 'recall': 0.14434994086000713,
 'support': 1930}
micro
{'f1-score': 0.7971899816737934,
 'precision': 0.9709821428571429,
 'recall': 0.6761658031088082,
 'support': 1930}
weighted
{'f1-score': 0.7352435789866867,
 'precision': 0.8627680282489885,
 'recall': 0.6761658031088082,
 'support': 1930}
```

## javascript
### Summary
10 rules, avg.len. 4.8

| | |
|-|-|
|Min support|94|
|Max support|2878|
|Min confidence|0.9204753041267395|
|Max confidence|0.9986263513565063|

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
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 2878.` |
| 2 | `  -1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 94.` |
| 3 | `  -1.reserved = =<br>	∧ ^1.roles in {OPERATOR} and not in {ADD, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 301.` |
| 4 | `  -1.reserved not in {(, =}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {ADD, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.920. Support: 547.` |
| 5 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 107.` |
| 6 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 1402.` |
| 7 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 481.` |
| 8 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 364.` |
| 9 | `  -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 151.` |
| 10 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.956. Support: 126.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.8, "max_conf": 0.9986263513565063, "max_support": 2878, "min_conf": 0.9204753041267395, "min_support": 94, "num_rules": 10}}
```
</details>
