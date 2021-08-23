# Model report for file:///tmp/top-repos-quality-repos-lwx9lnno/jsspider.git HEAD 8f6558280d1f2cdfd3fb10e79f4e3306e3eca90d

### Dump

```json
{'created_at': '2021-08-22 12:19:23',
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
 'size': '17.5 kB',
 'tags': [],
 'uuid': 'bc09638f-28ea-4ef8-aeb5-1ab4638bccfe',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-lwx9lnno/jsspider.git 8f6558280d1f2cdfd3fb10e79f4e3306e3eca90d

# javascript
32 rules, avg.len. 7.3
## train
PPCR: 0.948971
### report
macro
{'f1-score': 0.6991832028528955,
 'precision': 0.7132720336129872,
 'recall': 0.6865751211527805,
 'support': 63805}
micro
{'f1-score': 0.9785283285009012,
 'precision': 0.9785283285009012,
 'recall': 0.9785283285009012,
 'support': 63805}
weighted
{'f1-score': 0.9773890818512064,
 'precision': 0.9766429727280288,
 'recall': 0.9785283285009012,
 'support': 63805}
### report_full
macro
{'f1-score': 0.666336895843072,
 'precision': 0.7132720336129872,
 'recall': 0.627979865713997,
 'support': 67236}
micro
{'f1-score': 0.9529078685296969,
 'precision': 0.9785283285009012,
 'recall': 0.9285948004045452,
 'support': 67236}
weighted
{'f1-score': 0.946357115240345,
 'precision': 0.9666778666296298,
 'recall': 0.9285948004045452,
 'support': 67236}
## test
PPCR: 0.922173
### report
macro
{'f1-score': 0.6712181406323793,
 'precision': 0.6794627619718174,
 'recall': 0.6700822815615977,
 'support': 12951}
micro
{'f1-score': 0.977453478495869,
 'precision': 0.977453478495869,
 'recall': 0.977453478495869,
 'support': 12951}
weighted
{'f1-score': 0.9764975628442306,
 'precision': 0.9761636443439577,
 'recall': 0.977453478495869,
 'support': 12951}
### report_full
macro
{'f1-score': 0.6116059326902281,
 'precision': 0.6794627619718174,
 'recall': 0.5711153777815575,
 'support': 14044}
micro
{'f1-score': 0.9378773847008705,
 'precision': 0.977453478495869,
 'recall': 0.901381372828254,
 'support': 14044}
weighted
{'f1-score': 0.9242103495921895,
 'precision': 0.9529040980427754,
 'recall': 0.901381372828254,
 'support': 14044}
```

## javascript
### Summary
25 rules, avg.len. 6.6

| | |
|-|-|
|Min support|91|
|Max support|14055|
|Min confidence|0.9301075339317322|
|Max confidence|0.999886155128479|

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
| 1 | `  +1.reserved = )<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 713.` |
| 2 | `  +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 640.` |
| 3 | `  -1.reserved = (<br>	∧ +1.reserved not in {]}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 549.` |
| 4 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.reserved not in {[}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ]}<br>	∧ +3.reserved not in {=}<br>	∧ ^1.internal_type = BinaryExpression<br>	∧ ^1.roles in {BITWISE, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 1733.` |
| 5 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.reserved not in {[}<br>	∧ -3.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ]}<br>	∧ +3.reserved not in {=}<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles in {BITWISE, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 124.` |
| 6 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {LITERAL}<br>	∧ -2.reserved not in {[}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ]}<br>	∧ +3.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR} and not in {BITWISE}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 8399.` |
| 7 | `  -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 927.` |
| 8 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.997. Support: 151.` |
| 9 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {}}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 114.` |
| 10 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 14055.` |
| 11 | `  -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 4393.` |
| 12 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 3289.` |
| 13 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.995. Support: 501.` |
| 14 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = ForStatement<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 389.` |
| 15 | `  -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1200.` |
| 16 | `  -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.996. Support: 888.` |
| 17 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 632.` |
| 18 | `  -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 289.` |
| 19 | `  -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved = ,<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 200.` |
| 20 | `  -1.reserved not in {(, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.997. Support: 159.` |
| 21 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.930. Support: 93.` |
| 22 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 13<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 105.` |
| 23 | `  -1.label not in {<space>}<br>	∧ -1.reserved = ++<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 98.` |
| 24 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ -4.diff_col ≥ 9<br>	∧ -5.label in {<newline>}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.962. Support: 91.` |
| 25 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {,}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ ^1.internal_type not in {MemberExpression, NewExpression, SequenceExpression, VariableDeclaration}<br>	∧ ^1.roles not in {FOR, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 7688.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.6, "max_conf": 0.999886155128479, "max_support": 14055, "min_conf": 0.9301075339317322, "min_support": 91, "num_rules": 25}}
```
</details>
