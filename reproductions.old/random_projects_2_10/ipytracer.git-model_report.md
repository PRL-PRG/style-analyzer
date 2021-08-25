# Model report for file:///tmp/top-repos-quality-repos-_2gfujht/ipytracer.git HEAD fe6505f48a9c79ada9fec3bb61ab2969b0f30609

### Dump

```json
{'created_at': '2021-08-22 04:59:35',
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
 'size': '14.9 kB',
 'tags': [],
 'uuid': '5a2c9c41-e9d8-41a4-ab13-356ecafa0952',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-_2gfujht/ipytracer.git fe6505f48a9c79ada9fec3bb61ab2969b0f30609

# javascript
12 rules, avg.len. 3.1
## train
PPCR: 0.539277
### report
macro
{'f1-score': 0.37796745744858756,
 'precision': 0.405211617733386,
 'recall': 0.3689960205426976,
 'support': 1627}
micro
{'f1-score': 0.8617086662569146,
 'precision': 0.8617086662569146,
 'recall': 0.8617086662569146,
 'support': 1627}
weighted
{'f1-score': 0.8463859294954892,
 'precision': 0.8489386070539473,
 'recall': 0.8617086662569146,
 'support': 1627}
### report_full
macro
{'f1-score': 0.26581063647121345,
 'precision': 0.405211617733386,
 'recall': 0.2177135549199766,
 'support': 3017}
micro
{'f1-score': 0.6037898363479759,
 'precision': 0.8617086662569146,
 'recall': 0.4647000331455088,
 'support': 3017}
weighted
{'f1-score': 0.536993749522883,
 'precision': 0.7301964777088401,
 'recall': 0.4647000331455088,
 'support': 3017}
## test
PPCR: 0.419929
### report
macro
{'f1-score': 0.2996632996632997,
 'precision': 0.2810561757930179,
 'recall': 0.3249158249158249,
 'support': 118}
micro
{'f1-score': 0.9152542372881356,
 'precision': 0.9152542372881356,
 'recall': 0.9152542372881356,
 'support': 118}
weighted
{'f1-score': 0.8972778633795583,
 'precision': 0.8840321141837645,
 'recall': 0.9152542372881356,
 'support': 118}
### report_full
macro
{'f1-score': 0.2198036006546645,
 'precision': 0.2810561757930179,
 'recall': 0.18064182194616976,
 'support': 281}
micro
{'f1-score': 0.5413533834586466,
 'precision': 0.9152542372881356,
 'recall': 0.38434163701067614,
 'support': 281}
weighted
{'f1-score': 0.4736520842676669,
 'precision': 0.6174386683845383,
 'recall': 0.38434163701067614,
 'support': 281}
```

## javascript
### Summary
8 rules, avg.len. 3.0

| | |
|-|-|
|Min support|146|
|Max support|671|
|Min confidence|0.9213836193084717|
|Max confidence|0.9813711047172546|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.981. Support: 671.` |
| 2 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 628.` |
| 3 | `  -1.internal_type = Identifier<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 235.` |
| 4 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 611.` |
| 5 | `  -1.roles in {IDENTIFIER}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.944. Support: 223.` |
| 6 | `  -1.roles not in {LITERAL}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {CALL} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 146.` |
| 7 | `  -1.roles in {IDENTIFIER}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 243.` |
| 8 | `  -1.roles not in {LITERAL}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {CALL} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 159.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.0, "max_conf": 0.9813711047172546, "max_support": 671, "min_conf": 0.9213836193084717, "min_support": 146, "num_rules": 8}}
```
</details>
