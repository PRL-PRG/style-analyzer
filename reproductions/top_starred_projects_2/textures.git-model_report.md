# Model report for file:///tmp/top-repos-quality-repos-u2h6bxfm/textures.git HEAD bfde14ecf2c051c499ae25eac5f9eb5ed60a4eb2

### Dump

```json
{'created_at': '2021-08-30 03:54:04',
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
 'size': '15.7 kB',
 'tags': [],
 'uuid': '250042a2-433b-47c0-abd2-895ee2abb167',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-u2h6bxfm/textures.git bfde14ecf2c051c499ae25eac5f9eb5ed60a4eb2

# javascript
9 rules, avg.len. 4.7
## train
PPCR: 0.907866
### report
macro
{'f1-score': 0.6627565028950988,
 'precision': 0.6819720971383301,
 'recall': 0.6473816688588706,
 'support': 8021}
micro
{'f1-score': 0.940032414910859,
 'precision': 0.940032414910859,
 'recall': 0.940032414910859,
 'support': 8021}
weighted
{'f1-score': 0.9342499983987644,
 'precision': 0.931957969219977,
 'recall': 0.940032414910859,
 'support': 8021}
### report_full
macro
{'f1-score': 0.6057679244569466,
 'precision': 0.6819720971383301,
 'recall': 0.559533350984739,
 'support': 8835}
micro
{'f1-score': 0.8946369245372566,
 'precision': 0.940032414910859,
 'recall': 0.853423882286361,
 'support': 8835}
weighted
{'f1-score': 0.8698638611594728,
 'precision': 0.9000749288930837,
 'recall': 0.853423882286361,
 'support': 8835}
## test
PPCR: 0.916495
### report
macro
{'f1-score': 0.5091276306885444,
 'precision': 0.5155223224461885,
 'recall': 0.515173227185611,
 'support': 889}
micro
{'f1-score': 0.8222722159730033,
 'precision': 0.8222722159730034,
 'recall': 0.8222722159730034,
 'support': 889}
weighted
{'f1-score': 0.799045948533438,
 'precision': 0.7908888568664499,
 'recall': 0.8222722159730034,
 'support': 889}
### report_full
macro
{'f1-score': 0.47333973824657055,
 'precision': 0.5155223224461885,
 'recall': 0.4605314421411277,
 'support': 970}
micro
{'f1-score': 0.7864443249058634,
 'precision': 0.8222722159730034,
 'recall': 0.7536082474226804,
 'support': 970}
weighted
{'f1-score': 0.7518671942629808,
 'precision': 0.7662444159054752,
 'recall': 0.7536082474226804,
 'support': 970}
```

## javascript
### Summary
6 rules, avg.len. 4.7

| | |
|-|-|
|Min support|99|
|Max support|3840|
|Min confidence|0.9335378408432007|
|Max confidence|0.9990000128746033|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.999. Support: 500.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.internal_type = ArrowFunctionExpression<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 310.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^2.internal_type not in {ArrowFunctionExpression}<br>⇒ y = ⏎⇥⁻<br>Confidence: 0.995. Support: 99.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -4.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.length ≥ 3<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 234.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.length ≤ 2<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.934. Support: 489.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +5.length ≤ 16<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 3840.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.666666666666667, "max_conf": 0.9990000128746033, "max_support": 3840, "min_conf": 0.9335378408432007, "min_support": 99, "num_rules": 6}}
```
</details>
