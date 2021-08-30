# Model report for file:///tmp/top-repos-quality-repos-kgdr_bld/vue-h5-template.git HEAD c2109f0e7848b965a53c7076cd87706029ed7daa

### Dump

```json
{'created_at': '2021-08-29 21:53:14',
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
 'size': '14.8 kB',
 'tags': [],
 'uuid': '1a4f9b1c-2358-4c25-8926-35a031f0ede5',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-kgdr_bld/vue-h5-template.git c2109f0e7848b965a53c7076cd87706029ed7daa

# javascript
19 rules, avg.len. 4.6
## train
PPCR: 0.626906
### report
macro
{'f1-score': 0.4282364633664318,
 'precision': 0.4462311847320341,
 'recall': 0.43057520088529505,
 'support': 2302}
micro
{'f1-score': 0.8205907906168549,
 'precision': 0.8205907906168549,
 'recall': 0.8205907906168549,
 'support': 2302}
weighted
{'f1-score': 0.780934921128595,
 'precision': 0.7787653687127114,
 'recall': 0.8205907906168549,
 'support': 2302}
### report_full
macro
{'f1-score': 0.3555724211554688,
 'precision': 0.4462311847320341,
 'recall': 0.31865619429476966,
 'support': 3672}
micro
{'f1-score': 0.6324070974221627,
 'precision': 0.8205907906168549,
 'recall': 0.5144335511982571,
 'support': 3672}
weighted
{'f1-score': 0.5580601341614847,
 'precision': 0.6807332324409799,
 'recall': 0.5144335511982571,
 'support': 3672}
## test
PPCR: 0.450980
### report
macro
{'f1-score': 0.3512043512043512,
 'precision': 0.43197278911564624,
 'recall': 0.3703703703703704,
 'support': 69}
micro
{'f1-score': 0.7101449275362319,
 'precision': 0.7101449275362319,
 'recall': 0.7101449275362319,
 'support': 69}
weighted
{'f1-score': 0.6392689436167697,
 'precision': 0.7414965986394557,
 'recall': 0.7101449275362319,
 'support': 69}
### report_full
macro
{'f1-score': 0.24396776756327312,
 'precision': 0.43197278911564624,
 'recall': 0.21723856209150327,
 'support': 153}
micro
{'f1-score': 0.44144144144144143,
 'precision': 0.7101449275362319,
 'recall': 0.3202614379084967,
 'support': 153}
weighted
{'f1-score': 0.358293665894459,
 'precision': 0.6972122182206216,
 'recall': 0.3202614379084967,
 'support': 153}
```

## javascript
### Summary
9 rules, avg.len. 3.9

| | |
|-|-|
|Min support|130|
|Max support|259|
|Min confidence|0.924580991268158|
|Max confidence|0.9980695247650146|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.998. Support: 259.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 171.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.926. Support: 169.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 130.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.997. Support: 144.` |
| 6 | `  -1.roles in {STRING}<br>⇒ y = '<br>Confidence: 0.998. Support: 238.` |
| 7 | `  -1.reserved = :<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 167.` |
| 8 | `  -1.reserved not in {:}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 167.` |
| 9 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 179.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.888888888888889, "max_conf": 0.9980695247650146, "max_support": 259, "min_conf": 0.924580991268158, "min_support": 130, "num_rules": 9}}
```
</details>
