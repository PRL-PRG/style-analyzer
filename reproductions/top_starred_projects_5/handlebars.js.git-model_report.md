# Model report for file:///tmp/top-repos-quality-repos-ed2foeyb/handlebars.js.git HEAD 6790c080c641ef2b44e663800e1794fae180977a

### Dump

```json
{'created_at': '2021-08-30 00:00:53',
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
 'size': '22.3 kB',
 'tags': [],
 'uuid': '6d8bb240-9760-454b-b952-52f577e37766',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ed2foeyb/handlebars.js.git 6790c080c641ef2b44e663800e1794fae180977a

# javascript
156 rules, avg.len. 9.3
## train
PPCR: 0.943548
### report
macro
{'f1-score': 0.6695840643186035,
 'precision': 0.7091779935926665,
 'recall': 0.6410301475577257,
 'support': 49524}
micro
{'f1-score': 0.9264598982311606,
 'precision': 0.9264598982311606,
 'recall': 0.9264598982311606,
 'support': 49524}
weighted
{'f1-score': 0.9209046499356196,
 'precision': 0.9198031762151895,
 'recall': 0.9264598982311606,
 'support': 49524}
### report_full
macro
{'f1-score': 0.6062758581770871,
 'precision': 0.7091779935926665,
 'recall': 0.5511949822330149,
 'support': 52487}
micro
{'f1-score': 0.8995500485241787,
 'precision': 0.9264598982311606,
 'recall': 0.8741593156400632,
 'support': 52487}
weighted
{'f1-score': 0.8846574331804972,
 'precision': 0.9161448308663347,
 'recall': 0.8741593156400632,
 'support': 52487}
## test
PPCR: 0.950843
### report
macro
{'f1-score': 0.6108918611072491,
 'precision': 0.6467808961373913,
 'recall': 0.5988511121740157,
 'support': 12863}
micro
{'f1-score': 0.9088859519552204,
 'precision': 0.9088859519552204,
 'recall': 0.9088859519552204,
 'support': 12863}
weighted
{'f1-score': 0.89603401126057,
 'precision': 0.8926533407848575,
 'recall': 0.9088859519552204,
 'support': 12863}
### report_full
macro
{'f1-score': 0.5707241026996765,
 'precision': 0.6467808961373913,
 'recall': 0.5456142889949531,
 'support': 13528}
micro
{'f1-score': 0.8859838581334546,
 'precision': 0.9088859519552204,
 'recall': 0.8642075694855116,
 'support': 13528}
weighted
{'f1-score': 0.8610176659134532,
 'precision': 0.8819109293850526,
 'recall': 0.8642075694855116,
 'support': 13528}
```

## javascript
### Summary
94 rules, avg.len. 9.7

| | |
|-|-|
|Min support|139|
|Max support|12791|
|Min confidence|0.9240282773971558|
|Max confidence|0.9996697306632996|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.970. Support: 3519.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.962. Support: 588.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.length ≥ 7<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ⏎<br>Confidence: 0.924. Support: 283.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.diff_offset ≥ 13<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +5.reserved = (<br>⇒ y = '<br>Confidence: 0.961. Support: 241.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.diff_offset ≤ 12<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.970. Support: 2898.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 609.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.999. Support: 790.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 7607.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER, LITERAL}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.984. Support: 285.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {ARGUMENT, MAP}<br>	∧ -3.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 6319.` |
| 11 | `  •••start_col ≤ 17<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≥ 3<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 480.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 195.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_offset ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1514.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -2.reserved not in {}}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;, }}<br>	∧ +2.length ≥ 9<br>	∧ +5.roles in {IDENTIFIER}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.927. Support: 158.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.label in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ⏎<br>Confidence: 0.952. Support: 324.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.roles in {MAP}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 720.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 714.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {FUNCTION}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 548.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {FUNCTION}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 888.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.980. Support: 324.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {ARGUMENT, MAP}<br>	∧ -3.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 163.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {ARGUMENT, MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 11051.` |
| 23 | `  •••start_col ≤ 21<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.length ≥ 3<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 496.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 170.` |
| 25 | `  •••start_col ≥ 15<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 194.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {LITERAL, MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 3314.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.diff_offset ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>	∧ +2.reserved not in {:}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.977. Support: 1081.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -2.reserved not in {)}<br>	∧ -3.diff_col ≤ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 139.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -2.reserved not in {}}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;, }}<br>	∧ +2.length ≥ 9<br>	∧ +5.roles in {MAP}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.983. Support: 143.` |
| 30 | `  •••start_line ≥ 46<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.930. Support: 309.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.roles not in {VALUE}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.940. Support: 1289.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 851.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.roles in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {MAP} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 783.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 860.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +2.roles not in {MAP}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 1114.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles not in {MAP}<br>	∧ -2.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.997. Support: 181.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.length ≥ 2<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 359.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.length ≤ 2<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +2.reserved = =<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 201.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = return<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 203.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, return, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.reserved = }<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +2.reserved not in {=}<br>	∧ +2.length ≥ 9<br>	∧ +3.reserved = (<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.988. Support: 369.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, return, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.reserved not in {}}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +2.reserved not in {=}<br>	∧ +2.roles in {CALLEE}<br>	∧ +2.length ≥ 9<br>	∧ +3.reserved = (<br>	∧ +5.internal_type not in {Identifier}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 252.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, return, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +2.reserved not in {=}<br>	∧ +2.length ≤ 8<br>	∧ +3.reserved = (<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 1368.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 154.` |
| 44 | `  •••start_col ≤ 21<br>	∧ -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, return, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {(}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.938. Support: 251.` |
| 45 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, return, {, }}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {=}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 4082.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, return, {}<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {=}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 12510.` |
| 47 | `  •••start_col ≤ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 56<br>⇒ y = '<br>Confidence: 0.997. Support: 163.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = ObjectProperty<br>⇒ y = ␣<br>Confidence: 0.998. Support: 607.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 160.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 11089.` |
| 51 | `  •••start_col ≤ 19<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 481.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 163.` |
| 53 | `  •••start_col ≥ 17<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 178.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 3350.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.diff_offset ≤ 2<br>	∧ -3.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.987. Support: 980.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -2.reserved not in {}}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>	∧ +3.reserved = (<br>	∧ +5.roles in {IDENTIFIER}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.985. Support: 165.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {'}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 11492.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {'}<br>	∧ -1.reserved not in {,}<br>	∧ -1.length ≥ 3<br>	∧ -2.diff_offset ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 514.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {'}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 164.` |
| 60 | `  •••start_col ≥ 15<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {'}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 206.` |
| 61 | `  •••start_col ≤ 14<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {'}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 148.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {'}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 3337.` |
| 63 | `  •••start_col ≤ 35<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.928. Support: 367.` |
| 64 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {STATEMENT} and not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 334.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {STATEMENT} and not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 147.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,}<br>	∧ -2.diff_col ≤ 10<br>	∧ -2.diff_offset ≤ 45<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.963. Support: 178.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,}<br>	∧ -2.diff_offset ≤ 45<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 12791.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -3.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.951. Support: 1066.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -2.reserved not in {}}<br>	∧ -3.diff_col ≥ 9<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.reserved not in {)}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 1417.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -2.diff_offset ≤ 2<br>	∧ -2.reserved not in {}}<br>	∧ -3.diff_col ≤ 8<br>	∧ -3.diff_offset ≥ 5<br>	∧ -3.reserved not in {)}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 309.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -2.reserved not in {}}<br>	∧ -3.reserved not in {)}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {;}<br>	∧ +2.reserved not in {:}<br>	∧ +2.length ≥ 9<br>	∧ +5.roles in {IDENTIFIER}<br>	∧ ^2.roles not in {STATEMENT}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.947. Support: 159.` |
| 72 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {STATEMENT} and not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 346.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {STATEMENT} and not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 157.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {STATEMENT} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.955. Support: 1753.` |
| 75 | `  -1.diff_col ≤ 22<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 156.` |
| 76 | `  -1.diff_col ≤ 22<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 12621.` |
| 77 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label not in {<space>}<br>	∧ -3.diff_col ≤ 4<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 56<br>⇒ y = '<br>Confidence: 0.976. Support: 144.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_col ≤ 10<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {ARGUMENT, MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 159.` |
| 79 | `  •••start_col ≥ 15<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {LITERAL, MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 224.` |
| 80 | `  •••start_line ≥ 48<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.946. Support: 326.` |
| 81 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.label not in {<space>}<br>	∧ -3.diff_col ≤ 6<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≥ 55<br>⇒ y = '<br>Confidence: 0.956. Support: 193.` |
| 82 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 11325.` |
| 83 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ -2.diff_offset ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {STATEMENT}<br>	∧ ^2.roles in {STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 144.` |
| 84 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.926. Support: 331.` |
| 85 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles in {ARGUMENT} and not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 247.` |
| 86 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = var<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {ARGUMENT, MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 223.` |
| 87 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {var}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {ARGUMENT, MAP}<br>	∧ -3.label not in {'}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.roles in {EXPRESSION} and not in {OPERATOR}<br>	∧ ^2.internal_type = ObjectProperty<br>⇒ y = ∅<br>Confidence: 0.992. Support: 182.` |
| 88 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {var}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {ARGUMENT, MAP}<br>	∧ -3.label not in {'}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.roles in {EXPRESSION} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {ObjectProperty}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 11281.` |
| 89 | `  -1.diff_col ≤ 8<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {var}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 494.` |
| 90 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {ARGUMENT, MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR, SCOPE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 149.` |
| 91 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {var}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 176.` |
| 92 | `  •••start_col ≥ 15<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {var}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {ARGUMENT, MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 258.` |
| 93 | `  •••start_col ≤ 14<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {var}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {ARGUMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 165.` |
| 94 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {var}<br>	∧ -2.diff_offset ≥ 3<br>	∧ -2.roles not in {ARGUMENT, MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, =, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 3410.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.659574468085106, "max_conf": 0.9996697306632996, "max_support": 12791, "min_conf": 0.9240282773971558, "min_support": 139, "num_rules": 94}}
```
</details>
