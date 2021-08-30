# Model report for file:///tmp/top-repos-quality-repos-qjprgsnd/reactor.js.git HEAD 8b27230d591d77df490bac2bb9762b2201315f9a

### Dump

```json
{'created_at': '2021-08-29 21:57:44',
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
 'size': '14.0 kB',
 'tags': [],
 'uuid': '23c89496-5f7a-4933-9aab-2e18af2fd2b1',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-qjprgsnd/reactor.js.git 8b27230d591d77df490bac2bb9762b2201315f9a

# javascript
9 rules, avg.len. 5.8
## train
PPCR: 0.777370
### report
macro
{'f1-score': 0.5827338074938325,
 'precision': 0.5802593528076726,
 'recall': 0.5853267201920309,
 'support': 2116}
micro
{'f1-score': 0.968336483931947,
 'precision': 0.968336483931947,
 'recall': 0.968336483931947,
 'support': 2116}
weighted
{'f1-score': 0.9650240187544223,
 'precision': 0.9618361058843329,
 'recall': 0.968336483931947,
 'support': 2116}
### report_full
macro
{'f1-score': 0.4972212479922195,
 'precision': 0.5802593528076726,
 'recall': 0.4408267425425616,
 'support': 2722}
micro
{'f1-score': 0.847044233154196,
 'precision': 0.968336483931947,
 'recall': 0.7527553269654665,
 'support': 2722}
weighted
{'f1-score': 0.8159718811513655,
 'precision': 0.9052618171836221,
 'recall': 0.7527553269654665,
 'support': 2722}
## test
PPCR: 0.610889
### report
macro
{'f1-score': 0.5317118032763744,
 'precision': 0.5974088147497805,
 'recall': 0.49935483870967745,
 'support': 2906}
micro
{'f1-score': 0.996214728148658,
 'precision': 0.996214728148658,
 'recall': 0.996214728148658,
 'support': 2906}
weighted
{'f1-score': 0.9950171471983484,
 'precision': 0.9941717218530642,
 'recall': 0.996214728148658,
 'support': 2906}
### report_full
macro
{'f1-score': 0.31253756255062565,
 'precision': 0.5974088147497805,
 'recall': 0.2618039664850582,
 'support': 4757}
micro
{'f1-score': 0.7555787550567662,
 'precision': 0.996214728148658,
 'recall': 0.6085768341391633,
 'support': 4757}
weighted
{'f1-score': 0.7010110783732072,
 'precision': 0.939205169813055,
 'recall': 0.6085768341391633,
 'support': 4757}
```

## javascript
### Summary
8 rules, avg.len. 6.0

| | |
|-|-|
|Min support|97|
|Max support|506|
|Min confidence|0.9440789222717285|
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
| 1 | `  -1.internal_type = CommentLine<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 196.` |
| 2 | `  -1.internal_type not in {CommentLine}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.944. Support: 152.` |
| 3 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 506.` |
| 4 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 130.` |
| 5 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 208.` |
| 6 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 153.` |
| 7 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 155.` |
| 8 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -3.diff_offset ≥ 4<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 97.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.0, "max_conf": 0.9975961446762085, "max_support": 506, "min_conf": 0.9440789222717285, "min_support": 97, "num_rules": 8}}
```
</details>
