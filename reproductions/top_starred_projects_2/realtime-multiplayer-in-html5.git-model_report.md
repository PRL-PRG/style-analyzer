# Model report for file:///tmp/top-repos-quality-repos-d1qpf08f/realtime-multiplayer-in-html5.git HEAD 00af50dd57baa29f30809925881a624bc50234f1

### Dump

```json
{'created_at': '2021-08-30 03:46:06',
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
 'size': '17.1 kB',
 'tags': [],
 'uuid': '5f33e86c-b717-425b-9e45-d2b3ab627984',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-d1qpf08f/realtime-multiplayer-in-html5.git 00af50dd57baa29f30809925881a624bc50234f1

# javascript
7 rules, avg.len. 5.6
## train
PPCR: 0.690041
### report
macro
{'f1-score': 0.3979610770746865,
 'precision': 0.4064326846544518,
 'recall': 0.3970139932988171,
 'support': 5668}
micro
{'f1-score': 0.9518348623853211,
 'precision': 0.9518348623853211,
 'recall': 0.9518348623853211,
 'support': 5668}
weighted
{'f1-score': 0.9472952267607819,
 'precision': 0.9450434636822138,
 'recall': 0.9518348623853211,
 'support': 5668}
### report_full
macro
{'f1-score': 0.2754716978902775,
 'precision': 0.4064326846544518,
 'recall': 0.21985945337202714,
 'support': 8214}
micro
{'f1-score': 0.7772655236997551,
 'precision': 0.9518348623853211,
 'recall': 0.6568054541027514,
 'support': 8214}
weighted
{'f1-score': 0.7282148827165374,
 'precision': 0.8602469663727932,
 'recall': 0.6568054541027514,
 'support': 8214}
## test
PPCR: 0.681081
### report
macro
{'f1-score': 0.36755063903247376,
 'precision': 0.39168608181189196,
 'recall': 0.3552865857394349,
 'support': 378}
micro
{'f1-score': 0.9259259259259259,
 'precision': 0.9259259259259259,
 'recall': 0.9259259259259259,
 'support': 378}
weighted
{'f1-score': 0.9254543123750739,
 'precision': 0.9318969151603539,
 'recall': 0.9259259259259259,
 'support': 378}
### report_full
macro
{'f1-score': 0.24416306804007284,
 'precision': 0.39168608181189196,
 'recall': 0.1934886152652703,
 'support': 555}
micro
{'f1-score': 0.7502679528403002,
 'precision': 0.9259259259259259,
 'recall': 0.6306306306306306,
 'support': 555}
weighted
{'f1-score': 0.7046384590910172,
 'precision': 0.8812978743135054,
 'recall': 0.6306306306306306,
 'support': 555}
```

## javascript
### Summary
3 rules, avg.len. 5.0

| | |
|-|-|
|Min support|109|
|Max support|2154|
|Min confidence|0.9867687821388245|
|Max confidence|0.9986225962638855|

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
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 2154.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.995. Support: 109.` |
| 3 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, }}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 363.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.0, "max_conf": 0.9986225962638855, "max_support": 2154, "min_conf": 0.9867687821388245, "min_support": 109, "num_rules": 3}}
```
</details>
