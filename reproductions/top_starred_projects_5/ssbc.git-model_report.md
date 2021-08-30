# Model report for file:///tmp/top-repos-quality-repos-qmg5n5d_/ssbc.git HEAD 09a64ec201bf974add80db5408237d57fffa9135

### Dump

```json
{'created_at': '2021-08-30 00:24:45',
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
 'size': '15.2 kB',
 'tags': [],
 'uuid': '8e457c72-f08c-4eee-a520-6c03fad586eb',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-qmg5n5d_/ssbc.git 09a64ec201bf974add80db5408237d57fffa9135

# javascript
8 rules, avg.len. 3.6
## train
PPCR: 0.585376
### report
macro
{'f1-score': 0.7718959086982117,
 'precision': 0.7835796712246861,
 'recall': 0.7624485178996921,
 'support': 2698}
micro
{'f1-score': 0.9584877687175686,
 'precision': 0.9584877687175686,
 'recall': 0.9584877687175686,
 'support': 2698}
weighted
{'f1-score': 0.9569907963653789,
 'precision': 0.9565476978398519,
 'recall': 0.9584877687175686,
 'support': 2698}
### report_full
macro
{'f1-score': 0.5573256962691805,
 'precision': 0.7835796712246861,
 'recall': 0.4644998865180679,
 'support': 4609}
micro
{'f1-score': 0.7078144245244287,
 'precision': 0.9584877687175686,
 'recall': 0.5610761553482317,
 'support': 4609}
weighted
{'f1-score': 0.664143013981917,
 'precision': 0.925906403981895,
 'recall': 0.5610761553482317,
 'support': 4609}
## test
PPCR: 0.552506
### report
macro
{'f1-score': 0.749381408673444,
 'precision': 0.7876905583045933,
 'recall': 0.7324818391104487,
 'support': 463}
micro
{'f1-score': 0.937365010799136,
 'precision': 0.937365010799136,
 'recall': 0.937365010799136,
 'support': 463}
weighted
{'f1-score': 0.9321198177158725,
 'precision': 0.9419152702601604,
 'recall': 0.937365010799136,
 'support': 463}
### report_full
macro
{'f1-score': 0.5247450899979257,
 'precision': 0.7876905583045933,
 'recall': 0.4435189233137493,
 'support': 838}
micro
{'f1-score': 0.6671790930053805,
 'precision': 0.937365010799136,
 'recall': 0.5178997613365155,
 'support': 838}
weighted
{'f1-score': 0.6049279263638592,
 'precision': 0.9287891272660773,
 'recall': 0.5178997613365155,
 'support': 838}
```

## javascript
### Summary
5 rules, avg.len. 3.4

| | |
|-|-|
|Min support|114|
|Max support|798|
|Min confidence|0.9553571343421936|
|Max confidence|0.9963235259056091|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.961. Support: 798.` |
| 2 | `  -1.reserved = (<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 136.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.996. Support: 114.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 168.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 674.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.4, "max_conf": 0.9963235259056091, "max_support": 798, "min_conf": 0.9553571343421936, "min_support": 114, "num_rules": 5}}
```
</details>
