# Model report for file:///tmp/top-repos-quality-repos-xvu0zqbj/capstoneproj_courseme.git HEAD e6dceb2ac6887763a42295bdf6c3d7f0702769e0

### Dump

```json
{'created_at': '2021-08-22 11:52:49',
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
 'size': '17.7 kB',
 'tags': [],
 'uuid': '6732f5e6-1de1-44aa-9367-337543d4dbf7',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-xvu0zqbj/capstoneproj_courseme.git e6dceb2ac6887763a42295bdf6c3d7f0702769e0

# javascript
20 rules, avg.len. 4.7
## train
PPCR: 0.829130
### report
macro
{'f1-score': 0.2850446972255096,
 'precision': 0.2946076774523679,
 'recall': 0.2820607521381604,
 'support': 5687}
micro
{'f1-score': 0.8382275364867241,
 'precision': 0.8382275364867241,
 'recall': 0.8382275364867241,
 'support': 5687}
weighted
{'f1-score': 0.7963951732623982,
 'precision': 0.7718122289886507,
 'recall': 0.8382275364867241,
 'support': 5687}
### report_full
macro
{'f1-score': 0.26192283733012695,
 'precision': 0.2946076774523679,
 'recall': 0.24618514649667164,
 'support': 6859}
micro
{'f1-score': 0.7599234815877571,
 'precision': 0.8382275364867241,
 'recall': 0.6949992710307625,
 'support': 6859}
weighted
{'f1-score': 0.6957884289039031,
 'precision': 0.7271679953360555,
 'recall': 0.6949992710307625,
 'support': 6859}
## test
PPCR: 0.831800
### report
macro
{'f1-score': 0.28720599739044794,
 'precision': 0.2901563078706435,
 'recall': 0.2887483256924621,
 'support': 1266}
micro
{'f1-score': 0.8364928909952607,
 'precision': 0.8364928909952607,
 'recall': 0.8364928909952607,
 'support': 1266}
weighted
{'f1-score': 0.7916149380652866,
 'precision': 0.7597760811116604,
 'recall': 0.8364928909952607,
 'support': 1266}
### report_full
macro
{'f1-score': 0.2638669892466418,
 'precision': 0.2901563078706435,
 'recall': 0.2525351534983351,
 'support': 1522}
micro
{'f1-score': 0.7596843615494978,
 'precision': 0.8364928909952607,
 'recall': 0.6957950065703022,
 'support': 1522}
weighted
{'f1-score': 0.6958886486752388,
 'precision': 0.7230108184335136,
 'recall': 0.6957950065703022,
 'support': 1522}
```

## javascript
### Summary
9 rules, avg.len. 3.9

| | |
|-|-|
|Min support|143|
|Max support|1084|
|Min confidence|0.9773985147476196|
|Max confidence|0.9988937973976135|

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
                     'min_samples_leaf': 90,
                     'min_samples_split': 187,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.997. Support: 196.` |
| 2 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 1084.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 169.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 251.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.997. Support: 184.` |
| 6 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {IDENTIFIER} and not in {DECLARATION, MAP}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 452.` |
| 7 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, MAP}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 272.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 143.` |
| 9 | `  -1.internal_type = StringLiteral<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = '<br>Confidence: 0.997. Support: 166.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.888888888888889, "max_conf": 0.9988937973976135, "max_support": 1084, "min_conf": 0.9773985147476196, "min_support": 143, "num_rules": 9}}
```
</details>
