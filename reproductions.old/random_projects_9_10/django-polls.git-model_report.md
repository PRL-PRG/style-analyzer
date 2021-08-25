# Model report for file:///tmp/top-repos-quality-repos-kfvbti2n/django-polls.git HEAD c99f95871aee715298d1e7c7538734cd6f8b3c6d

### Dump

```json
{'created_at': '2021-08-20 20:01:27',
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
 'size': '17.2 kB',
 'tags': [],
 'uuid': '8981bcac-7c3a-45d4-ac8b-4fbdbab04b28',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-kfvbti2n/django-polls.git c99f95871aee715298d1e7c7538734cd6f8b3c6d

# javascript
19 rules, avg.len. 6.0
## train
PPCR: 0.956513
### report
macro
{'f1-score': 0.6893949526220304,
 'precision': 0.7001586853229815,
 'recall': 0.6817766032115021,
 'support': 14671}
micro
{'f1-score': 0.9405630154727013,
 'precision': 0.9405630154727013,
 'recall': 0.9405630154727013,
 'support': 14671}
weighted
{'f1-score': 0.9321822269878578,
 'precision': 0.9251827898095484,
 'recall': 0.9405630154727013,
 'support': 14671}
### report_full
macro
{'f1-score': 0.6646982848384273,
 'precision': 0.7001586853229815,
 'recall': 0.6386739128863781,
 'support': 15338}
micro
{'f1-score': 0.9196574361025025,
 'precision': 0.9405630154727013,
 'recall': 0.8996609727474247,
 'support': 15338}
weighted
{'f1-score': 0.9104172544646036,
 'precision': 0.9242060427221828,
 'recall': 0.8996609727474247,
 'support': 15338}
## test
PPCR: 0.961496
### report
macro
{'f1-score': 0.6553626493922698,
 'precision': 0.6434315774835685,
 'recall': 0.6932931311766632,
 'support': 2597}
micro
{'f1-score': 0.889872930304197,
 'precision': 0.8898729303041971,
 'recall': 0.8898729303041971,
 'support': 2597}
weighted
{'f1-score': 0.8698254658297183,
 'precision': 0.8608847756361321,
 'recall': 0.8898729303041971,
 'support': 2597}
### report_full
macro
{'f1-score': 0.6332817182928088,
 'precision': 0.6434315774835685,
 'recall': 0.6529522545637885,
 'support': 2701}
micro
{'f1-score': 0.8724046810117025,
 'precision': 0.8898729303041971,
 'recall': 0.8556090336912254,
 'support': 2701}
weighted
{'f1-score': 0.853570816711017,
 'precision': 0.8634954082701494,
 'recall': 0.8556090336912254,
 'support': 2701}
```

## javascript
### Summary
14 rules, avg.len. 6.4

| | |
|-|-|
|Min support|91|
|Max support|4351|
|Min confidence|0.9285714030265808|
|Max confidence|0.9972826242446899|

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
| 1 | `  -1.roles in {STRING}<br>⇒ y = '<br>Confidence: 0.936. Support: 1188.` |
| 2 | `  -1.roles not in {STRING}<br>	∧ -4.reserved = '<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 750.` |
| 3 | `  -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 456.` |
| 4 | `  -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.970. Support: 217.` |
| 5 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 413.` |
| 6 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.969. Support: 275.` |
| 7 | `  -1.reserved = ,<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 279.` |
| 8 | `  -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 274.` |
| 9 | `  -1.reserved = var<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 184.` |
| 10 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, ;, var, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.942. Support: 146.` |
| 11 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 91.` |
| 12 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = if<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 91.` |
| 13 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, :, ;, if, var, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_line ≥ 1<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 657.` |
| 14 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, :, ;, if, var, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_line = 0<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 4351.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.428571428571429, "max_conf": 0.9972826242446899, "max_support": 4351, "min_conf": 0.9285714030265808, "min_support": 91, "num_rules": 14}}
```
</details>
