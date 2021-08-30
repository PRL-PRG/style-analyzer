# Model report for file:///tmp/top-repos-quality-repos-a4ywgfc9/npm-check-updates.git HEAD cf0a52ca7d569422072954903081f6f8b4f8a39c

### Dump

```json
{'created_at': '2021-08-29 12:27:01',
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
 'size': '17.6 kB',
 'tags': [],
 'uuid': '5360cf5f-c95e-49bf-9f55-5595f23d66f0',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-a4ywgfc9/npm-check-updates.git cf0a52ca7d569422072954903081f6f8b4f8a39c

# javascript
26 rules, avg.len. 8.3
## train
PPCR: 0.936727
### report
macro
{'f1-score': 0.753539892721162,
 'precision': 0.7814813466791661,
 'recall': 0.7382810660831629,
 'support': 28114}
micro
{'f1-score': 0.9439069502738849,
 'precision': 0.9439069502738849,
 'recall': 0.9439069502738849,
 'support': 28114}
weighted
{'f1-score': 0.9408841734398056,
 'precision': 0.9422400470193922,
 'recall': 0.9439069502738849,
 'support': 28114}
### report_full
macro
{'f1-score': 0.6917454006562896,
 'precision': 0.7814813466791661,
 'recall': 0.6479470734613881,
 'support': 30013}
micro
{'f1-score': 0.9130696578182256,
 'precision': 0.9439069502738849,
 'recall': 0.884183520474461,
 'support': 30013}
weighted
{'f1-score': 0.9034718346119698,
 'precision': 0.9401829131387105,
 'recall': 0.884183520474461,
 'support': 30013}
## test
PPCR: 0.948497
### report
macro
{'f1-score': 0.7221644251874677,
 'precision': 0.7685980962090553,
 'recall': 0.7038789067574607,
 'support': 6722}
micro
{'f1-score': 0.9418327878607557,
 'precision': 0.9418327878607557,
 'recall': 0.9418327878607557,
 'support': 6722}
weighted
{'f1-score': 0.9356979103845716,
 'precision': 0.9373950946553115,
 'recall': 0.9418327878607557,
 'support': 6722}
### report_full
macro
{'f1-score': 0.6697165871903267,
 'precision': 0.7685980962090553,
 'recall': 0.6327112991220216,
 'support': 7087}
micro
{'f1-score': 0.9169382286914332,
 'precision': 0.9418327878607557,
 'recall': 0.89332580781713,
 'support': 7087}
weighted
{'f1-score': 0.9022278001141562,
 'precision': 0.9343520031311734,
 'recall': 0.89332580781713,
 'support': 7087}
```

## javascript
### Summary
16 rules, avg.len. 6.9

| | |
|-|-|
|Min support|97|
|Max support|4251|
|Min confidence|0.9251269102096558|
|Max confidence|0.9997214674949646|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 1.000. Support: 1795.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.996. Support: 384.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.925. Support: 394.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.981. Support: 1781.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = )<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 97.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {)}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 4251.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 570.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 3619.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 1364.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 527.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 301.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 311.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 250.` |
| 14 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ), {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≥ 18<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.962. Support: 171.` |
| 15 | `  •••start_col ≤ 30<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 17<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {,, >, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.928. Support: 214.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ), ,, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 17<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 114.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.875, "max_conf": 0.9997214674949646, "max_support": 4251, "min_conf": 0.9251269102096558, "min_support": 97, "num_rules": 16}}
```
</details>
