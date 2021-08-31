# Model report for file:///tmp/top-repos-quality-repos-8bl0ufyj/calla.git HEAD 956548753edd684092f507fe528cd5f02ff52da9

### Dump

```json
{'created_at': '2021-08-31 12:38:15',
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
 'uuid': '8ead0020-bd3b-490b-8527-5ffa08f7bcf6',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-8bl0ufyj/calla.git 956548753edd684092f507fe528cd5f02ff52da9

# javascript
44 rules, avg.len. 7.5
## train
PPCR: 0.992201
### report
macro
{'f1-score': 0.7260619200941065,
 'precision': 0.7316677704867105,
 'recall': 0.7207917816208098,
 'support': 185350}
micro
{'f1-score': 0.9854761262476396,
 'precision': 0.9854761262476396,
 'recall': 0.9854761262476396,
 'support': 185350}
weighted
{'f1-score': 0.9843144845470533,
 'precision': 0.9832122106764032,
 'recall': 0.9854761262476396,
 'support': 185350}
### report_full
macro
{'f1-score': 0.7153197691950162,
 'precision': 0.7316677704867105,
 'recall': 0.7008450341360277,
 'support': 186807}
micro
{'f1-score': 0.981617973059757,
 'precision': 0.9854761262476396,
 'recall': 0.9777899115129519,
 'support': 186807}
weighted
{'f1-score': 0.9803518444381512,
 'precision': 0.9831578356058238,
 'recall': 0.9777899115129519,
 'support': 186807}
## test
PPCR: 0.972379
### report
macro
{'f1-score': 0.611499442445183,
 'precision': 0.6412269013437792,
 'recall': 0.6015357835174068,
 'support': 40097}
micro
{'f1-score': 0.8814624535501409,
 'precision': 0.8814624535501409,
 'recall': 0.8814624535501409,
 'support': 40097}
weighted
{'f1-score': 0.8882709183702261,
 'precision': 0.9041715378976615,
 'recall': 0.8814624535501409,
 'support': 40097}
### report_full
macro
{'f1-score': 0.5920739038113918,
 'precision': 0.6412269013437792,
 'recall': 0.5709925818593662,
 'support': 41236}
micro
{'f1-score': 0.8691183160586723,
 'precision': 0.8814624535501409,
 'recall': 0.857115142108837,
 'support': 41236}
weighted
{'f1-score': 0.8741425695366738,
 'precision': 0.9034282022336809,
 'recall': 0.857115142108837,
 'support': 41236}
```

## javascript
### Summary
36 rules, avg.len. 7.0

| | |
|-|-|
|Min support|105|
|Max support|37005|
|Min confidence|0.9206008315086365|
|Max confidence|0.9999054074287415|

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
                     'min_samples_leaf': 91,
                     'min_samples_split': 181,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.990. Support: 12527.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.length ≥ 2<br>⇒ y = ⏎<br>Confidence: 0.989. Support: 8562.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.993. Support: 4888.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>⇒ y = ␣<br>Confidence: 0.980. Support: 625.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 2<br>⇒ y = "<br>Confidence: 0.986. Support: 12499.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 1.000. Support: 5285.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 3347.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 359.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.960. Support: 2122.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles in {BODY}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.950. Support: 1481.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -5.diff_offset ≤ 10<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 6<br>	∧ +3.length ≥ 4<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 800.` |
| 12 | `  •••start_line ≥ 154<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -5.diff_offset ≤ 10<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 6<br>	∧ +3.length ≤ 3<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 116.` |
| 13 | `  •••start_col ≥ 29<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -5.diff_offset ≤ 10<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 5<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.978. Support: 160.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 646.` |
| 15 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 400.` |
| 16 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -5.diff_line ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≤ 2<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 271.` |
| 17 | `  •••start_col ≤ 52<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL} and not in {FILE}<br>⇒ y = ⏎<br>Confidence: 0.921. Support: 233.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL} and not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 114.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 232.` |
| 20 | `  •••start_col ≥ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 35092.` |
| 21 | `  •••start_col ≤ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -3.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 184.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.998. Support: 8107.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.991. Support: 3204.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.roles in {ARGUMENT}<br>	∧ -4.label in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.932. Support: 229.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.roles not in {ARGUMENT}<br>	∧ -4.label in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.925. Support: 796.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.label not in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 124.` |
| 27 | `  •••start_col ≥ 57<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -4.label not in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = export<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 105.` |
| 28 | `  •••start_col ≤ 56<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -4.label not in {<space>}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.966. Support: 1694.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 206.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), {, }}<br>	∧ +1.length ≤ 1<br>	∧ +3.length ≤ 2<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 1398.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.964. Support: 827.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.999. Support: 660.` |
| 33 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>⇒ y = ␣<br>Confidence: 0.957. Support: 128.` |
| 34 | `  •••start_col ≥ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 37005.` |
| 35 | `  •••start_col ≤ 12<br>	∧ -1.internal_type = Identifier<br>	∧ -1.reserved not in {,, if}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 139.` |
| 36 | `  •••start_col ≤ 12<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 797.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.972222222222222, "max_conf": 0.9999054074287415, "max_support": 37005, "min_conf": 0.9206008315086365, "min_support": 105, "num_rules": 36}}
```
</details>
