# Model report for file:///tmp/top-repos-quality-repos-ih5dwxey/maria.git HEAD a13f0367247bb850da38f11bbc7906302e939954

### Dump

```json
{'created_at': '2021-08-29 16:13:43',
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
 'size': '18.0 kB',
 'tags': [],
 'uuid': 'e5aa18b8-6092-46ef-99a7-5572036cdf32',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ih5dwxey/maria.git a13f0367247bb850da38f11bbc7906302e939954

# javascript
36 rules, avg.len. 8.9
## train
PPCR: 0.945477
### report
macro
{'f1-score': 0.8983932981699605,
 'precision': 0.9264887760156904,
 'recall': 0.8811577707654592,
 'support': 56167}
micro
{'f1-score': 0.9638043691135365,
 'precision': 0.9638043691135364,
 'recall': 0.9638043691135364,
 'support': 56167}
weighted
{'f1-score': 0.9631234467526231,
 'precision': 0.9641816620611598,
 'recall': 0.9638043691135364,
 'support': 56167}
### report_full
macro
{'f1-score': 0.8137039188799561,
 'precision': 0.9264887760156904,
 'recall': 0.759912102099459,
 'support': 59406}
micro
{'f1-score': 0.9367931956425809,
 'precision': 0.9638043691135364,
 'recall': 0.9112547554119113,
 'support': 59406}
weighted
{'f1-score': 0.9297423149003914,
 'precision': 0.9610274833170034,
 'recall': 0.9112547554119113,
 'support': 59406}
## test
PPCR: 0.940140
### report
macro
{'f1-score': 0.7751710971881455,
 'precision': 0.8273584130012102,
 'recall': 0.7782316827228863,
 'support': 9015}
micro
{'f1-score': 0.9467554076539102,
 'precision': 0.9467554076539102,
 'recall': 0.9467554076539102,
 'support': 9015}
weighted
{'f1-score': 0.9494328589362111,
 'precision': 0.9581694192821066,
 'recall': 0.9467554076539102,
 'support': 9015}
### report_full
macro
{'f1-score': 0.6942180943447145,
 'precision': 0.8273584130012102,
 'recall': 0.65540926210486,
 'support': 9589}
micro
{'f1-score': 0.9175446140614922,
 'precision': 0.9467554076539102,
 'recall': 0.8900823860673689,
 'support': 9589}
weighted
{'f1-score': 0.9117623207155967,
 'precision': 0.9529771446621681,
 'recall': 0.8900823860673689,
 'support': 9589}
```

## javascript
### Summary
22 rules, avg.len. 8.0

| | |
|-|-|
|Min support|114|
|Max support|8625|
|Min confidence|0.9334394931793213|
|Max confidence|0.9991582632064819|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.995. Support: 8625.` |
| 2 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 231.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved not in {)}<br>	∧ +1.internal_type = Identifier<br>	∧ +3.roles in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 159.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved not in {)}<br>	∧ +3.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 4516.` |
| 5 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 704.` |
| 6 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 6550.` |
| 7 | `  •••start_line ≤ 244<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.996. Support: 114.` |
| 8 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 3145.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 2393.` |
| 10 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.961. Support: 1557.` |
| 11 | `  -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.933. Support: 1570.` |
| 12 | `  -1.reserved = ;<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FOR} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 137.` |
| 13 | `  •••start_line ≤ 254<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {EXPRESSION} and not in {IDENTIFIER}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.943. Support: 326.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {EXPRESSION} and not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 718.` |
| 15 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 594.` |
| 16 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles in {LITERAL} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 289.` |
| 17 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles in {FILE} and not in {LITERAL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 406.` |
| 18 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {COMMENT} and not in {EXPRESSION, IDENTIFIER}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.roles in {FILE} and not in {LITERAL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 149.` |
| 19 | `  •••start_line ≤ 254<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, LITERAL, OPERATOR}<br>⇒ y = '<br>Confidence: 0.934. Support: 221.` |
| 20 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {FILE, LITERAL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 130.` |
| 21 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 7143.` |
| 22 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ -2.reserved = (<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {), ,, }}<br>	∧ ^1.roles not in {FILE, LITERAL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 116.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.954545454545454, "max_conf": 0.9991582632064819, "max_support": 8625, "min_conf": 0.9334394931793213, "min_support": 114, "num_rules": 22}}
```
</details>
