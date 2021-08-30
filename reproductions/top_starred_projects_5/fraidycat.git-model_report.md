# Model report for file:///tmp/top-repos-quality-repos-w7b9_hmc/fraidycat.git HEAD a5480cfaf37953463598892f977ce53069a40a69

### Dump

```json
{'created_at': '2021-08-29 22:29:31',
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
 'size': '19.0 kB',
 'tags': [],
 'uuid': 'b1fcfd05-e3cc-427f-9982-3a198ec7a647',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-w7b9_hmc/fraidycat.git a5480cfaf37953463598892f977ce53069a40a69

# javascript
127 rules, avg.len. 7.4
## train
PPCR: 0.936112
### report
macro
{'f1-score': 0.644751255221238,
 'precision': 0.746315466577544,
 'recall': 0.5985462037389992,
 'support': 22125}
micro
{'f1-score': 0.8825310734463276,
 'precision': 0.8825310734463276,
 'recall': 0.8825310734463276,
 'support': 22125}
weighted
{'f1-score': 0.8738431651749157,
 'precision': 0.877701157968016,
 'recall': 0.8825310734463276,
 'support': 22125}
### report_full
macro
{'f1-score': 0.5709574281703849,
 'precision': 0.746315466577544,
 'recall': 0.5179758054770575,
 'support': 23635}
micro
{'f1-score': 0.8534090909090909,
 'precision': 0.8825310734463276,
 'recall': 0.8261476623651365,
 'support': 23635}
weighted
{'f1-score': 0.8291778848996537,
 'precision': 0.8708225594199643,
 'recall': 0.8261476623651365,
 'support': 23635}
## test
PPCR: 0.893285
### report
macro
{'f1-score': 0.42927502503770965,
 'precision': 0.5702471624850657,
 'recall': 0.380449723391515,
 'support': 745}
micro
{'f1-score': 0.8375838926174497,
 'precision': 0.8375838926174497,
 'recall': 0.8375838926174497,
 'support': 745}
weighted
{'f1-score': 0.8144446106885671,
 'precision': 0.8172902985254142,
 'recall': 0.8375838926174497,
 'support': 745}
### report_full
macro
{'f1-score': 0.36014055363908726,
 'precision': 0.5702471624850657,
 'recall': 0.3210820077737993,
 'support': 834}
micro
{'f1-score': 0.7903736542115264,
 'precision': 0.8375838926174497,
 'recall': 0.7482014388489209,
 'support': 834}
weighted
{'f1-score': 0.7343220997474791,
 'precision': 0.8036146351735816,
 'recall': 0.7482014388489209,
 'support': 834}
```

## javascript
### Summary
75 rules, avg.len. 7.3

| | |
|-|-|
|Min support|136|
|Max support|4605|
|Min confidence|0.9207119941711426|
|Max confidence|0.9995689392089844|

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
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 3417.` |
| 2 | `  +1.roles not in {STRING}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.937. Support: 725.` |
| 3 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 749.` |
| 4 | `  -1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 458.` |
| 5 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 392.` |
| 6 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.926. Support: 2863.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {INCOMPLETE} and not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 753.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 470.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 287.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 183.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ><br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 138.` |
| 12 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.973. Support: 3266.` |
| 13 | `  +1.roles not in {STRING}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 779.` |
| 14 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 769.` |
| 15 | `  -1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 441.` |
| 16 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.993. Support: 352.` |
| 17 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXAttribute<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = "<br>Confidence: 0.952. Support: 136.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles in {STATEMENT} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.988. Support: 303.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 246.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {ConditionalExpression, MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 4220.` |
| 21 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 3236.` |
| 22 | `  -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.970. Support: 654.` |
| 23 | `  -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.934. Support: 492.` |
| 24 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 706.` |
| 25 | `  -1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 503.` |
| 26 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.991. Support: 378.` |
| 27 | `  +1.length ≤ 1<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 475.` |
| 28 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 2910.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 927.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 453.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 284.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 213.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 146.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, =}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 584.` |
| 35 | `  ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 1499.` |
| 36 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1160.` |
| 37 | `  -1.reserved not in {(}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 356.` |
| 38 | `  -1.reserved not in {(}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement, VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 2606.` |
| 39 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.993. Support: 383.` |
| 40 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 348.` |
| 41 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {INCOMPLETE}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 177.` |
| 42 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 742.` |
| 43 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.990. Support: 334.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {STATEMENT} and not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 344.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 226.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 4605.` |
| 47 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 719.` |
| 48 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.roles in {IDENTIFIER}<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR, SCOPE}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 140.` |
| 49 | `  +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.933. Support: 518.` |
| 50 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 2927.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.924. Support: 819.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 409.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 242.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 210.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ><br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 160.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.length ≥ 3<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 694.` |
| 57 | `  +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 807.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {STATEMENT} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 357.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 219.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 4197.` |
| 61 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.999. Support: 350.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {STATEMENT} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 306.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 1653.` |
| 64 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.roles not in {COMMENT, IDENTIFIER}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXAttribute<br>	∧ ^1.roles not in {BLOCK, OPERATOR}<br>⇒ y = "<br>Confidence: 0.946. Support: 140.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles in {INCOMPLETE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 138.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 2592.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {if}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {CONDITION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 1559.` |
| 68 | `  -1.internal_type = CommentLine<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.989. Support: 416.` |
| 69 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 2949.` |
| 70 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1035.` |
| 71 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 347.` |
| 72 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = "<br>Confidence: 0.962. Support: 143.` |
| 73 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 219.` |
| 74 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, =, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ,<br>	∧ ^1.internal_type not in {JSXElement, MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 208.` |
| 75 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, =, {}<br>	∧ -1.roles not in {EXPRESSION, IDENTIFIER}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 420.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.266666666666667, "max_conf": 0.9995689392089844, "max_support": 4605, "min_conf": 0.9207119941711426, "min_support": 136, "num_rules": 75}}
```
</details>
