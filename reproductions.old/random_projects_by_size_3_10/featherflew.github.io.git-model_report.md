# Model report for file:///tmp/top-repos-quality-repos-zhrgqjyr/featherflew.github.io.git HEAD 6c55e8fb2447c369eb92e28d939573551d4ed62f

### Dump

```json
{'created_at': '2021-08-22 01:54:38',
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
 'uuid': '41558d6a-0a2f-4ab8-b1fa-6a053dc04063',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-zhrgqjyr/featherflew.github.io.git 6c55e8fb2447c369eb92e28d939573551d4ed62f

# javascript
19 rules, avg.len. 3.2
## train
PPCR: 0.496126
### report
macro
{'f1-score': 0.2535577465720444,
 'precision': 0.2446635106564463,
 'recall': 0.2632177423277078,
 'support': 2305}
micro
{'f1-score': 0.8711496746203905,
 'precision': 0.8711496746203905,
 'recall': 0.8711496746203905,
 'support': 2305}
weighted
{'f1-score': 0.8430757602478761,
 'precision': 0.8169902122713141,
 'recall': 0.8711496746203905,
 'support': 2305}
### report_full
macro
{'f1-score': 0.18273656149014053,
 'precision': 0.2446635106564463,
 'recall': 0.14938628266039292,
 'support': 4646}
micro
{'f1-score': 0.5777585958854842,
 'precision': 0.8711496746203905,
 'recall': 0.4321997417133018,
 'support': 4646}
weighted
{'f1-score': 0.5253550486379625,
 'precision': 0.6953664863022375,
 'recall': 0.4321997417133018,
 'support': 4646}
## test
PPCR: 0.558290
### report
macro
{'f1-score': 0.2451060342344316,
 'precision': 0.23010691143945336,
 'recall': 0.2647296475151147,
 'support': 431}
micro
{'f1-score': 0.8422273781902552,
 'precision': 0.8422273781902552,
 'recall': 0.8422273781902552,
 'support': 431}
weighted
{'f1-score': 0.8063659446224783,
 'precision': 0.7783527841251884,
 'recall': 0.8422273781902552,
 'support': 431}
### report_full
macro
{'f1-score': 0.18980255384339578,
 'precision': 0.23010691143945336,
 'recall': 0.16307615439063997,
 'support': 772}
micro
{'f1-score': 0.6034912718204488,
 'precision': 0.8422273781902552,
 'recall': 0.47020725388601037,
 'support': 772}
weighted
{'f1-score': 0.5405849534340795,
 'precision': 0.6431947501730575,
 'recall': 0.47020725388601037,
 'support': 772}
```

## javascript
### Summary
12 rules, avg.len. 3.1

| | |
|-|-|
|Min support|156|
|Max support|595|
|Min confidence|0.9294871687889099|
|Max confidence|0.9967948794364929|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.964. Support: 595.` |
| 2 | `  +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 234.` |
| 3 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = (<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 156.` |
| 4 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 566.` |
| 5 | `  -1.roles not in {LITERAL}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 183.` |
| 6 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 587.` |
| 7 | `  +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 232.` |
| 8 | `  +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 209.` |
| 9 | `  +1.reserved not in {;}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {ARGUMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 189.` |
| 10 | `  -1.roles not in {LITERAL}<br>	∧ +1.reserved = ;<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 192.` |
| 11 | `  +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 245.` |
| 12 | `  +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 218.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.0833333333333335, "max_conf": 0.9967948794364929, "max_support": 595, "min_conf": 0.9294871687889099, "min_support": 156, "num_rules": 12}}
```
</details>
