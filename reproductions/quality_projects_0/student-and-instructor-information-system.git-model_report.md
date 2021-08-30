# Model report for file:///tmp/top-repos-quality-repos-1p5yybar/student-and-instructor-information-system.git HEAD b8a3ea744f01f038e2ca13dd6d45ea638d082051

### Dump

```json
{'created_at': '2021-08-29 01:25:45',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.19.0-12-amd64-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '21.1 kB',
 'tags': [],
 'uuid': '1a1ba2b7-5bf5-4c1a-abde-2eb89dde574f',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-1p5yybar/student-and-instructor-information-system.git b8a3ea744f01f038e2ca13dd6d45ea638d082051

# javascript
132 rules, avg.len. 7.9
## train
PPCR: 0.947029
### report
macro
{'f1-score': 0.5766579269983899,
 'precision': 0.6146679060419334,
 'recall': 0.5545491773657034,
 'support': 28516}
micro
{'f1-score': 0.9183616215457988,
 'precision': 0.9183616215457988,
 'recall': 0.9183616215457988,
 'support': 28516}
weighted
{'f1-score': 0.9109550994975006,
 'precision': 0.9092406002854319,
 'recall': 0.9183616215457988,
 'support': 28516}
### report_full
macro
{'f1-score': 0.5392828100923941,
 'precision': 0.6146679060419334,
 'recall': 0.49639463609383894,
 'support': 30111}
micro
{'f1-score': 0.8933767717945655,
 'precision': 0.9183616215457988,
 'recall': 0.8697153864036399,
 'support': 30111}
weighted
{'f1-score': 0.8811845901845676,
 'precision': 0.9033179487459438,
 'recall': 0.8697153864036399,
 'support': 30111}
## test
PPCR: 0.949503
### report
macro
{'f1-score': 0.5762117607507126,
 'precision': 0.6171273192235149,
 'recall': 0.5487610473954181,
 'support': 6788}
micro
{'f1-score': 0.9107248084855628,
 'precision': 0.9107248084855628,
 'recall': 0.9107248084855628,
 'support': 6788}
weighted
{'f1-score': 0.9064124331252061,
 'precision': 0.9070030044173568,
 'recall': 0.9107248084855628,
 'support': 6788}
### report_full
macro
{'f1-score': 0.5394724339315584,
 'precision': 0.6171273192235149,
 'recall': 0.4899481526419794,
 'support': 7149}
micro
{'f1-score': 0.8871349644830306,
 'precision': 0.9107248084855628,
 'recall': 0.8647363267589873,
 'support': 7149}
weighted
{'f1-score': 0.877823969234578,
 'precision': 0.9004536502053435,
 'recall': 0.8647363267589873,
 'support': 7149}
```

## javascript
### Summary
54 rules, avg.len. 6.9

| | |
|-|-|
|Min support|151|
|Max support|4490|
|Min confidence|0.9202127456665039|
|Max confidence|0.9987804889678955|

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
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 255.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -2.label in {<space>}<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 223.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.995. Support: 4438.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 3825.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 710.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.roles not in {IMPORT}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY, STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {FILE, IDENTIFIER, SCOPE}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 1098.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 2929.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -3.reserved not in {{}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 1406.` |
| 9 | `  -1.reserved = :<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 234.` |
| 10 | `  -1.reserved not in {:}<br>	∧ -2.label in {<space>}<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 224.` |
| 11 | `  -1.internal_type = StringLiteral<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles not in {STRING}<br>⇒ y = "<br>Confidence: 0.923. Support: 771.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.994. Support: 3771.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 678.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.roles not in {IMPORT}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY, STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, JSXElement, MemberExpression}<br>	∧ ^1.roles not in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 1134.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≥ 3<br>	∧ -2.roles not in {IMPORT}<br>	∧ +1.roles not in {EXPRESSION, KEY, STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, JSXElement, MemberExpression}<br>	∧ ^1.roles not in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 583.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 205.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^2.internal_type = JSXOpeningElement<br>⇒ y = ∅<br>Confidence: 0.997. Support: 169.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 2891.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.932. Support: 169.` |
| 20 | `  -1.reserved not in {,, :}<br>	∧ -2.label not in {<space>}<br>	∧ -3.diff_offset ≥ 12<br>	∧ +1.roles in {STRING}<br>	∧ ^2.roles in {MAP}<br>⇒ y = '<br>Confidence: 0.937. Support: 151.` |
| 21 | `  •••start_line ≥ 47<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles in {STRING}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = "<br>Confidence: 0.928. Support: 711.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.948. Support: 165.` |
| 23 | `  -1.reserved = :<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.993. Support: 214.` |
| 24 | `  -1.reserved not in {:}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.938. Support: 217.` |
| 25 | `  -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.993. Support: 4490.` |
| 26 | `  -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 3979.` |
| 27 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 713.` |
| 28 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_col ≥ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.920. Support: 282.` |
| 29 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.933. Support: 171.` |
| 30 | `  -1.roles in {IDENTIFIER} and not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 2847.` |
| 31 | `  -1.roles not in {IDENTIFIER, STRING}<br>	∧ -3.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 1415.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 3780.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 741.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {BODY, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.926. Support: 155.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -2.roles not in {IMPORT}<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, JSXElement, ObjectExpression}<br>	∧ ^1.roles not in {BODY, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 1099.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≥ 3<br>	∧ -2.roles not in {IMPORT}<br>	∧ +1.roles not in {EXPRESSION, STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {File, JSXElement, ObjectExpression}<br>	∧ ^1.roles not in {BODY, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 579.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 410.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 385.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -3.reserved not in {{}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {DECLARATION} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 704.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 2430.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {)}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 443.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ +3.reserved not in {{}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 2826.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.960. Support: 3991.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 373.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 337.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -4.label not in {<space>}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 693.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 238.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 216.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.internal_type = JSXAttribute<br>⇒ y = "<br>Confidence: 0.968. Support: 363.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 3953.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 445.` |
| 52 | `  -2.label in {<space>}<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 242.` |
| 53 | `  -1.reserved = :<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles in {STRING}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 223.` |
| 54 | `  -1.reserved not in {:}<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles in {STRING}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = "<br>Confidence: 0.964. Support: 372.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.944444444444445, "max_conf": 0.9987804889678955, "max_support": 4490, "min_conf": 0.9202127456665039, "min_support": 151, "num_rules": 54}}
```
</details>