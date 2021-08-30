# Model report for file:///tmp/top-repos-quality-repos-tngnhgr_/karma-coverage.git HEAD 6b1419fb2b14c24f367ef9718975870531b129d2

### Dump

```json
{'created_at': '2021-08-29 22:05:53',
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
 'size': '15.8 kB',
 'tags': [],
 'uuid': '7e0af62a-c672-4afc-b859-4e7645e9325e',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-tngnhgr_/karma-coverage.git 6b1419fb2b14c24f367ef9718975870531b129d2

# javascript
12 rules, avg.len. 5.6
## train
PPCR: 0.819895
### report
macro
{'f1-score': 0.6578651134603145,
 'precision': 0.6528081902073587,
 'recall': 0.6654336904914254,
 'support': 6264}
micro
{'f1-score': 0.9529054916985952,
 'precision': 0.9529054916985952,
 'recall': 0.9529054916985952,
 'support': 6264}
weighted
{'f1-score': 0.9468038706136681,
 'precision': 0.9421949592870403,
 'recall': 0.9529054916985952,
 'support': 6264}
### report_full
macro
{'f1-score': 0.6091743981411345,
 'precision': 0.6528081902073587,
 'recall': 0.5791075522650445,
 'support': 7640}
micro
{'f1-score': 0.858601841196778,
 'precision': 0.9529054916985952,
 'recall': 0.7812827225130891,
 'support': 7640}
weighted
{'f1-score': 0.8202345275825959,
 'precision': 0.8725749519006427,
 'recall': 0.7812827225130891,
 'support': 7640}
## test
PPCR: 0.842697
### report
macro
{'f1-score': 0.6506939556595349,
 'precision': 0.6417031400143852,
 'recall': 0.6621182943044124,
 'support': 1500}
micro
{'f1-score': 0.92, 'precision': 0.92, 'recall': 0.92, 'support': 1500}
weighted
{'f1-score': 0.9067663721717718,
 'precision': 0.895863402312251,
 'recall': 0.92,
 'support': 1500}
### report_full
macro
{'f1-score': 0.6170864814285124,
 'precision': 0.6417031400143852,
 'recall': 0.6023963407750249,
 'support': 1780}
micro
{'f1-score': 0.8414634146341463,
 'precision': 0.92,
 'recall': 0.7752808988764045,
 'support': 1780}
weighted
{'f1-score': 0.8054793412871758,
 'precision': 0.8454294138772948,
 'recall': 0.7752808988764045,
 'support': 1780}
```

## javascript
### Summary
9 rules, avg.len. 6.0

| | |
|-|-|
|Min support|98|
|Max support|1445|
|Min confidence|0.9506410360336304|
|Max confidence|0.998908281326294|

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
                     'min_samples_leaf': 94,
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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 1445.` |
| 2 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 220.` |
| 3 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.995. Support: 98.` |
| 4 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 458.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 205.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 134.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ,<br>⇒ y = ∅<br>Confidence: 0.995. Support: 111.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.internal_type not in {BlockStatement, File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 780.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.diff_offset ≤ 3<br>	∧ -3.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.internal_type not in {BlockStatement, File}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 105.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.0, "max_conf": 0.998908281326294, "max_support": 1445, "min_conf": 0.9506410360336304, "min_support": 98, "num_rules": 9}}
```
</details>
